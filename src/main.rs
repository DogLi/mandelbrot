extern crate num;
extern crate image;
extern crate num_cpus;
extern crate crossbeam;
extern crate rayon;

use std::io;
use std::path::Path;
use std::io::Write;
use num::Complex;
use std::str::FromStr;
use image::{Rgb, Pixel, ImageBuffer};
use image::png::PNGEncoder;
use std::fs::File;
use rayon::prelude::*;


const R: f64 = 4.0;
const ITER_NUM: u32 = 200;
const COLOR_LEN: usize = 3;

pub struct Picture<'a> {
    buffer: ImageBuffer<Rgb<u8>, Vec<u8>>,
    path_name: &'a str,
}

impl <'a> Picture<'a> {
    fn new(width: usize, height: usize, name: &'a str) -> Self {
        // There's probably a cleaner way to figure out the right size
        let storage = vec![0; COLOR_LEN * width * height];
        let buf = ImageBuffer::from_raw(width as u32, height as u32, storage).unwrap();

        Picture {
            buffer: buf,
            path_name: name,
        }
    }

    fn fill_color(&mut self, pixels: &[Rgb<u8>]) -> io::Result<()> {
        for (index, p) in self.buffer.pixels_mut().enumerate() {
            let color = pixels[index];
            *p = color;
        }
        let path = Path::new(self.path_name);
        self.buffer.save(path)
    }
}

#[allow(dead_code)]
fn complex_squre_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

/// Try to determine if `c` is in the Mandelbrot set, using at most `limit`
/// iterations to decide.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius two centered on the
/// origin. If `c` seems to be a member (more precisely, if we reached the
/// iteration limit without being able to prove that `c` is not a member),
/// return `None`.


fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > R {
            return Some(i);
        }
    }
    None
}

/// get rgb color
/// From: https://www.reddit.com/r/math/comments/2abwyt/smooth_colour_mandelbrot/

fn color_rgb(z: Complex<f64>, i: Option<u32>)->Rgb<u8>
{
    let data = match i {
        None => {
            [0 as u8, 0 as u8 , 0 as u8]
        },
        Some(i) => {
            // log2(i + R - np.log2(np.log2(abs(z)))) / 5
            let mut v = (i as f64 + R- z.norm_sqr().log2().log2()).log2() / 5.0;

            if v < 1.0 {
                // v**4, v**2.5, v
                [(v.powi(4) * 255.0) as u8, (v.powf(2.5) * 255.0) as u8, (v * 255.0) as u8]
            } else {
                // v = max(0, 2 - v); v, v**1.5, v**3
                v = (2.0 - v).max(0.0);
                [(v * 255.0) as u8, (v.powf(1.5) * 255.0) as u8, (v.powi(3) * 255.0) as u8]

            }
        }
    };
    Rgb{data: data}
}

/// Parse the string `s` as a coordinate pair, like `"400x600"` or `1.0, 0.5`
///
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is
/// the character given by the `separator` argument, and <left> and <right> are both
/// strings that can be parsed by `T::from_str`.
///
/// If `s` has the proper form, return `Some<x,y>` or else return `None`.
///
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
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
fn parse_complex(s: &str) -> Option<Complex<f64>> {
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
fn pixel2point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    begin_complex: Complex<f64>,
    end_complex: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        end_complex.re - begin_complex.re,
        end_complex.im - begin_complex.im,
    );
    Complex {
        re: begin_complex.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: begin_complex.im + pixel.1 as f64 * height / bounds.1 as f64,
    }
}

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
            let point = pixel2point(bounds, (column, row), upper_left, lower_right);
            let count = escape_time(point, ITER_NUM);
            pixels[row * bounds.0 + column] = color_rgb(point, count);
        }
    }
}



/// Write the buffer `pixels`, whose dimensions are given by `bounds`,
/// to the file named `filename`.
///
fn write_image(filename: &str, pixels: &[Rgb<u8>], bounds: (usize, usize)) -> io::Result<()> {
    let mut pic = Picture::new(bounds.0, bounds.1, filename);
    pic.fill_color(pixels)
}

fn do_parallel_render_crossbeam(
    pixels: &mut Vec<Rgb<u8>>,
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    let threads = num_cpus::get();
    let rows_per_band = bounds.1 / threads + 1;

    let bands: Vec<&mut [Rgb<u8>]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
    crossbeam::scope(|spawner| for (i, band) in bands.into_iter().enumerate() {
        let top = rows_per_band * i;
        let height = band.len() / bounds.0;
        let band_bounds = (bounds.0, height);
        let band_upper_left = pixel2point(bounds, (0, top), upper_left, lower_right);
        let band_lower_right =
            pixel2point(bounds, (bounds.0, top + height), upper_left, lower_right);
        spawner.spawn(move || {
            render(band, band_bounds, band_upper_left, band_lower_right)
        });
    });
}

fn do_parallel_render_rayon(
    pixels: &mut Vec<Rgb<u8>>,
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    let threads = num_cpus::get();
    let rows_per_band = bounds.1 / threads + 1;

    let bands: Vec<(usize, &mut [Rgb<u8>])> = pixels.chunks_mut(bounds.0).enumerate().collect();
    bands.into_par_iter().for_each(|(i, band)| {
        let top = i;
        let band_bounds = (bounds.0, 1);
        let band_upper_left = pixel2point(bounds, (0, top), upper_left, lower_right);
        let band_lower_right = pixel2point(bounds, (bounds.0, top + 1), upper_left, lower_right);
        render(band, band_bounds, band_upper_left, band_lower_right);
    });
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        writeln!(
            std::io::stderr(),
            "Usage: mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT"
        ).unwrap();
        writeln!(
            std::io::stderr(),
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
//            "Example: {} mandel.png 1000x750 -2.5,-1.2 1,1.20",

            args[0]
        ).unwrap();
        std::process::exit(1)
    }

    let bounds = parse_pair::<usize>(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing right corner point");
    let mut pixels = vec![Rgb{data: [0 as u8; 3]}; bounds.0 * bounds.1];
        render(&mut pixels, bounds, upper_left, lower_right);
    //    do_parallel_render_crossbeam(&mut pixels, bounds, upper_left, lower_right);
//    do_parallel_render_rayon(&mut pixels, bounds, upper_left, lower_right);
    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}


#[cfg(test)]
mod tests {
    use super::*;
    use num::Complex;
    #[test]
    fn test_color() {
        let c = Complex { re: -1.20, im: 0.35};
        let count = escape_time(c, ITER_NUM);
        println!("{:?}", count);
        assert!(true);
    }

    #[test]
    fn test_parse_pair() {
        assert_eq!(parse_pair::<i32>("", ','), None);
        assert_eq!(parse_pair::<i32>("10,", ','), None);
        assert_eq!(parse_pair::<i32>(",10", ','), None);
        assert_eq!(parse_pair::<i32>("5,10", ','), Some((5, 10)));
        assert_eq!(parse_pair::<i32>("5,10ab", ','), None);
        assert_eq!(parse_pair::<f64>("0.5x", ','), None);
        assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
    }

    #[test]
    fn test_pars_complex() {
        assert_eq!(
            parse_complex("1.25,-0.025"),
            Some(Complex {
                re: 1.25,
                im: -0.025,
            })
        );
        assert_eq!(parse_complex(",-0,1"), None);
    }

    #[test]
    fn test_pixel2point() {
        assert_eq!(
            pixel2point(
                (100, 100),
                (25, 75),
                Complex { re: -1.0, im: 1.0 },
                Complex { re: 1.0, im: -1.0 },
            ),
            Complex { re: -0.5, im: -0.5 }
        )
    }
}
