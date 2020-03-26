/**
 * Generic Plotter
 *
 * A generic heatmap generator for use in big data environments when
 * files get large enough that you cannot hold the whole thing in memory
 *
 * GNU GPL v2+
 *
 * 2020, Jan Christian Kässens <j.kaessens@ikmb.uni-kiel.de>
 * Institute for Clinical Molecular Biology
 * University Hospital Schleswig-Holstein Kiel, Germany
 */

#[macro_use]
extern crate clap;

extern crate plotters;

use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::{App, Arg};
use plotters::prelude::*;

/// How much to read at once, in bytes
const BUFFER_SIZE: usize = 64 * 1024 * 1024;

/// Color lightness for min and max values
const MIN_L: f64 = 0.3;
const MAX_L: f64 = 1.0;

// Parses a [0..1] coordinate pair from a line of text
fn parse_line(line: &str, xcol: usize, ycol: usize) -> (f32, f32) {
    let fields: Vec<&str> = line.split_whitespace().collect();

    if max(xcol, ycol) > fields.len() {
        panic!("No such field: {}", max(xcol, ycol));
    } else {
        (
            fields[xcol].parse::<f32>().unwrap(),
            fields[ycol].parse::<f32>().unwrap(),
        )
    }
}

/// Draws a heat map with axis descriptions
///
/// # Arguments
///
/// * 'data' - An (unsorted) list of counters
/// * 'size' - The size of the target image in pixels
/// * 'xdescr' - X axis description
/// * 'ydescr' - Y axis description
/// * 'target' - Filename for target
fn draw_heatmap(
    data: Vec<u64>,
    size: (u32, u32),
    xdescr: &str,
    ydescr: &str,
    target: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let bitmap = BitMapBackend::new(target, size).into_drawing_area();
    bitmap.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&bitmap)
        .margin(10)
        .x_label_area_size(35)
        .y_label_area_size(50)
        .build_ranged(0f32..1f32, 0f32..1f32)?;

    // set up mesh and axis description
    chart
        .configure_mesh()
        .x_desc(xdescr)
        .y_desc(ydescr)
        .draw()?;

    let max = (*data.iter().max().unwrap() as f64).log10();

    // draw points of data vector
    chart.draw_series(data.into_iter().enumerate().filter_map(|(idx, count)| {
        if count > 0 {
            let y = (idx as f32 / size.0 as f32) as u32;
            let x = (idx as f32 - (y * size.0) as f32) as u32;

            let x_pos = x as f32 / size.0 as f32;
            let y_pos = y as f32 / size.1 as f32;
            let lightness = ((count as f64).log10() / max as f64) * (MAX_L - MIN_L) + MIN_L;
            let color = &HSLColor(0.0 / 360.0, 1.0, lightness);
            Some(Circle::new((x_pos, y_pos), 3, color.filled()))
        } else {
            None
        }
    }))?;

    Ok(())
}

/// Reads coordinate pairs from a file into an array
fn load_data(
    source: &str,
    xcol: usize,
    ycol: usize,
    size: (u32, u32),
) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    let mut v = vec![0u64; size.0 as usize * size.1 as usize];

    // Calculate bin sizes for x and y, based on the pixel size of the target
    // image.
    let bin_x_size = 1f32 / (size.0 - 1) as f32;
    let bin_y_size = 1f32 / (size.1 - 1) as f32;

    let reader = BufReader::with_capacity(BUFFER_SIZE, File::open(source)?);

    for line in reader.lines().skip(1) {
        let pair = parse_line(&(line.unwrap()), xcol, ycol);
        let x_bin = (pair.0 / bin_x_size) as usize;
        let y_bin = (pair.1 / bin_y_size) as usize;
        v[y_bin * size.1 as usize + x_bin] += 1;
    }

    Ok(v)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up CLI args
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("source")
                .value_name("FILE")
                .takes_value(true)
                .required(true)
                .help("Whitespace-separated file with values to plot"),
        )
        .arg(
            Arg::with_name("target")
                .value_name("FILE")
                .takes_value(true)
                .required(true)
                .help("Plot output image file name (.png)"),
        )
        .arg(
            Arg::with_name("x")
                .short("x")
                .long("x")
                .takes_value(true)
                .value_name("COLUMN")
                .help("Select column for X axis"),
        )
        .arg(
            Arg::with_name("y")
                .short("y")
                .long("y")
                .takes_value(true)
                .value_name("COLUMN")
                .help("Select column for Y axis"),
        )
        .arg(
            Arg::with_name("xdesc")
                .long("xdesc")
                .takes_value(true)
                .value_name("COLNAME")
                .help("Axis description for X axis"),
        )
        .arg(
            Arg::with_name("ydesc")
                .long("ydesc")
                .takes_value(true)
                .value_name("COLNAME")
                .help("Axis description for Y axis"),
        )
        .arg(
            Arg::with_name("xsize")
                .long("xsize")
                .takes_value(true)
                .value_name("PIXELS")
                .help("Size in pixels (X axis)"),
        )
        .arg(
            Arg::with_name("ysize")
                .long("ysize")
                .takes_value(true)
                .value_name("PIXELS")
                .help("Size in pixels (Y axis)"),
        )
        .get_matches();

    let size = (
        matches
            .value_of("xsize")
            .unwrap_or("800")
            .parse::<u32>()
            .unwrap(),
        matches
            .value_of("ysize")
            .unwrap_or("800")
            .parse::<u32>()
            .unwrap(),
    );

    let data = load_data(
        matches.value_of("source").unwrap(),
        matches
            .value_of("x")
            .unwrap_or("6")
            .parse::<usize>()
            .unwrap(),
        matches
            .value_of("y")
            .unwrap_or("7")
            .parse::<usize>()
            .unwrap(),
        size,
    )?;

    draw_heatmap(
        data,
        size,
        matches.value_of("xdesc").unwrap_or("Z0"),
        matches.value_of("ydesc").unwrap_or("Z1"),
        matches.value_of("target").unwrap(),
    )
}
