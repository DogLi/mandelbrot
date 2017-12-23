extern crate num;
extern crate image;
extern crate rayon;


pub mod color;
pub mod img;
pub mod parser;
pub mod render;

use num::Complex;
use img::Picture;
use image::Rgb;



/// Write the buffer `pixels`, whose dimensions are given by `bounds`,
/// to the file named `filename`.
///
pub fn write_image(filename: &str, pixels: &[Rgb<u8>], bounds: (usize, usize)) {
    let mut pic = Picture::new(bounds.0, bounds.1, filename);
    pic.fill_color(pixels);
}

pub fn run(bounds: (usize, usize), begin_point: Complex<f64>, end_point: Complex<f64>, file: &str) {
    let mut pixels = vec![color::BLACK_COLOR; bounds.0 * bounds.1];
    render::do_parallel_render(&mut pixels, bounds, begin_point, end_point);
    write_image(file, &pixels, bounds);
}
