use num::Complex;
use image::Rgb;
use rayon::prelude::*;
use super::{parser, color};

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
/// The `bounds` argument gives the width and height of the buffer `pixels`,
/// which holds one grayscale pixel per byte. The `upper_left` and `lower_right`
/// arguments specify points on the complex plane corresponding to the upper-left
/// and lower-right corners ot the pixel buffer.
///
fn render(
    pixels: &mut [Rgb<u8>],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert_eq!(pixels.len(), bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let c = parser::pixel2point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = color::get_color_rgb(c);
        }
    }
}

pub fn do_parallel_render(
    pixels: &mut Vec<Rgb<u8>>,
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    let bands: Vec<(usize, &mut [Rgb<u8>])> = pixels.chunks_mut(bounds.0).enumerate().collect();
    bands.into_par_iter().for_each(|(i, band)| {
        let top = i;
        let band_bounds = (bounds.0, 1);
        let band_upper_left = parser::pixel2point(bounds, (0, top), upper_left, lower_right);
        let band_lower_right = parser::pixel2point(bounds, (bounds.0, top + 1), upper_left, lower_right);
        render(band, band_bounds, band_upper_left, band_lower_right);
    });
}