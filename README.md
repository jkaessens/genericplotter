![Rust build](https://github.com/ikmb/genericplotter/workflows/Rust/badge.svg?branch=master)

# genericplotter
A generic plotting module to plot almost arbitrary values into rasterized images

```
genericplotter 0.2.0
Jan Christian Kaessens <j.kaessens@ikmb.uni-kiel.de>
A generic scatterplot generator

USAGE:
    genericplotter [OPTIONS] <FILE> <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -x, --x <COLUMN>         Select column for X axis
        --xdesc <COLNAME>    Axis description for X axis
        --xsize <PIXELS>     Size in pixels (X axis)
    -y, --y <COLUMN>         Select column for Y axis
        --ydesc <COLNAME>    Axis description for Y axis
        --ysize <PIXELS>     Size in pixels (Y axis)

ARGS:
    <FILE>    Whitespace-separated file with values to plot
    <FILE>    Plot output image file name (.png)
```
