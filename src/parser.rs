use num::Complex;
use std::str::FromStr;

/// Parse the string `s` as a coordinate pair, like `"400x600"` or `1.0, 0.5`
///
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is
/// the character given by the `separator` argument, and <left> and <right> are both
/// strings that can be parsed by `T::from_str`.
///
/// If `s` has the proper form, return `Some<x,y>` or else return `None`.
///
pub fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None,
            }
        }
    }
}


/// Parse a pair of floating-point numbers separated by a comma as a complex number
///
pub fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}


/// Given the row and colum of a pixel in the output image, return the corresponding point on the
/// complex plane.
///
/// `bound` is a pair giving the width and height of the image in pixels.
/// `pixel` is a (column, row) pair indecating a particular pixel in that image.
/// The `upper_left` and `lower_right` parameters are points on the complex plane
///
pub fn pixel2point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    begin_complex: Complex<f64>,
    end_complex: Complex<f64>,
) -> Complex<f64> {
    Complex {
        re: begin_complex.re +
            (end_complex.re - begin_complex.re) * pixel.0 as f64 / bounds.0 as f64,
        im: begin_complex.im +
            (end_complex.im - begin_complex.im) * pixel.1 as f64 / bounds.1 as f64,
    }
}
