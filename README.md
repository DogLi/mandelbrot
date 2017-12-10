# Mandelbrot Set
Use pure Rust to generate beautiful Mandelbrot Set.

## Requirements:
- Rust >= 1.22
- num
- image
- rayon
- clap

## Generate the Standard Mandelbrot Set:

Run:

```
$ cargo build && cargo run -- --help
USAGE:
    mandelbrot --begin_point <begin_point> --end_point <end_point> --file <file> --pixels <pixels>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --begin_point <begin_point>    set the begin complex value which re and im are separated by ','  [default:
                                       -1.20,0.35]
    -e, --end_point <end_point>        set the end complex value which re and im are separated by ','  [default:
                                       -1,0.20]
    -f, --file <file>                  set the file path!
    -p, --pixels <pixels>              set the picture pixels! [default: 1200x960]
```

and check "mandelbrot.png"

Use the setting:
```
-b=-1.20,0.35 -e=-1,0.20
```
Will get:
[](https://github.com/DogLi/mandelbrot/blob/master/img/a.png)

Use the setting:
```
-b=-2.5,-1.2 -e=1,1.2
```
Will get:
[](https://github.com/DogLi/mandelbrot/blob/master/img/b.png)

```
-b=-0.090,0.654 -e=-0.086,0.657
```
Will get:
[](https://github.com/DogLi/mandelbrot/blob/master/img/c.png)

```
-b=-0.750,0.099 -e=-0.747,0.102
```
Will get:
[](https://github.com/DogLi/mandelbrot/blob/master/img/d.png)

```
-b=0.275,0.006 -e=0.28,0.01
```
Will get:
[](https://github.com/DogLi/mandelbrot/blob/master/img/e.png)

Try more settings by yourself!

# Thanks
Thanks to [tensorflow-fractal-playground](https://github.com/hzy46/tensorflow-fractal-playground)

