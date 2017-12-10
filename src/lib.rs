extern crate num;
extern crate image;
extern crate rayon;


pub mod color;
pub mod img;
pub mod parser;
pub mod render;

use num::Complex;

pub fn run(bounds: (usize, usize), begin_point: Complex<f64>, end_point: Complex<f64>, file: &str){
    let mut pixels = vec![color::BLACK_COLOR; bounds.0 * bounds.1];
    render::do_parallel_render(&mut pixels, bounds, begin_point, end_point);
    img::write_image(file, &pixels, bounds);
}