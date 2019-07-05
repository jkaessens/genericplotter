#[macro_use]
extern crate clap;

extern crate plotters;

use std::io::{BufReader, BufRead};
use std::fs::File;
use std::cmp::max;

use plotters::prelude::*;
use clap::{App, Arg};


const BUFFER_SIZE :usize = 1024*1024;

// Parses a [0..1] coordinate pair from a line of text
fn parse_line(line: &str, xcol: usize, ycol: usize) -> (f32, f32) {
    let fields: Vec<&str> = line.split_whitespace().collect();

    if max(xcol, ycol) > fields.len() {
        panic!("No such field: {}", max(xcol, ycol));
    } else {
        (fields[xcol].parse::<f32>().unwrap(),fields[ycol].parse::<f32>().unwrap())
    }
}

fn draw_plot(source: &str,
             xcol: usize, ycol: usize,
             xdescr: &str, ydescr: &str,
             title: &str, target: &str, size: (u32, u32)) -> Result<(), Box<dyn std::error::Error>> {

    let reader = BufReader::with_capacity(BUFFER_SIZE,File::open(source)?);

    // Set up canvas
    let bitmap = BitMapBackend::new(target, size).into_drawing_area();
    bitmap.fill(&White)?;

    // Set up caption and ranges
    let mut chart = ChartBuilder::on(&bitmap)
        .caption(title, ("Open Sans", 35).into_font())
        .margin(10)
        .x_label_area_size(35)
        .y_label_area_size(50)
        .build_ranged(0f32..1f32, 0f32..1f32)?;

    chart.configure_mesh()
        .axis_desc_style(("Open Sans", 25).into_font())
        .x_desc(xdescr)
        .y_desc(ydescr)
        .draw()?;

    // Read file line-by-line, skip the first one
    let iter = reader.lines().skip(1);

    // Make a line iterator to map each line to a (x, y) pair
    let point_iter = iter.map(
        |line| {
            let pair = parse_line(&(line.unwrap()), xcol, ycol);
            Circle::new(pair, 2, Blue.filled())
        });

    // Draw series
    chart.draw_series(point_iter)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Set up CLI args
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("source")
            .value_name("FILE")
            .takes_value(true)
            .required(true)
            .help("Whitespace-separated file with values to plot"))
        .arg(Arg::with_name("target")
            .value_name("FILE")
            .takes_value(true)
            .required(true)
            .help("Plot output image file name (.png)"))
        .arg(Arg::with_name("x")
            .short("x")
            .long("x")
            .takes_value(true)
            .value_name("COLUMN")
            .help("Select column for X axis"))
        .arg(Arg::with_name("y")
            .short("y")
            .long("y")
            .takes_value(true)
            .value_name("COLUMN")
            .help("Select column for Y axis"))
        .arg(Arg::with_name("xdesc")
            .long("xdesc")
            .takes_value(true)
            .value_name("COLNAME")
            .help("Axis description for X axis"))
        .arg(Arg::with_name("ydesc")
            .long("ydesc")
            .takes_value(true)
            .value_name("COLNAME")
            .help("Axis description for Y axis"))
        .arg(Arg::with_name("title")
            .long("title")
            .takes_value(true)
            .value_name("TITLE")
            .help("Plot title"))
        .arg(Arg::with_name("xsize")
            .long("xsize")
            .takes_value(true)
            .value_name("PIXELS")
            .help("Size in pixels (X axis)"))
        .arg(Arg::with_name("ysize")
            .long("ysize")
            .takes_value(true)
            .value_name("PIXELS")
            .help("Size in pixels (Y axis)"))
        .get_matches();

    // run plotter
    draw_plot(matches.value_of("source").unwrap(),
              matches.value_of("x").unwrap_or("6").parse::<usize>().unwrap(),
              matches.value_of("y").unwrap_or("7").parse::<usize>().unwrap(),
              matches.value_of("xdesc").unwrap_or("Z0"),
              matches.value_of("ydesc").unwrap_or("Z1"),
              matches.value_of("title").unwrap_or("IBD/IBS Plot"),
              matches.value_of("source").unwrap(),
              (
                  matches.value_of("xsize").unwrap_or("800").parse::<u32>().unwrap(),
                  matches.value_of("xsize").unwrap_or("600").parse::<u32>().unwrap()
              ))
}
