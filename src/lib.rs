extern crate num;
extern crate image;
extern crate rayon;

pub mod color;
pub mod img;
pub mod parser;
pub mod render;

use std::io::Write;

pub fn run(){
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        writeln!(
            std::io::stderr(),
            "Usage: mandelbrot FILE PIXELS STARTPOINT ENDPOINT"
        ).unwrap();
        writeln!(
            std::io::stderr(),
            r#"Example: {} mandel.png 1200x960 -1.20,0.35 -1,0.20 and you can try the point:
         -2.5,-1.2 1,1.2
         0.275,0.006 0.28,0.01
         -0.090,0.654 -0.086,0.657
         -0.750,0.099 -0.747,0.102
         "#,
//            "Example: {} mandel.png 1200x960 -2.5,-1.2 1,1.20",

            args[0]
        ).unwrap();
        std::process::exit(1)
    }

    let bounds = parser::parse_pair::<usize>(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parser::parse_complex(&args[3]).expect("error parsing left corner point");
    let lower_right = parser::parse_complex(&args[4]).expect("error parsing right corner point");
    let mut pixels = vec![color::BLACK_COLOR; bounds.0 * bounds.1];
    render::do_parallel_render(&mut pixels, bounds, upper_left, lower_right);
    img::write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}