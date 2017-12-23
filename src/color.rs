use num::Complex;
use image::Rgb;

pub const R: f64 = 4.0;
pub const RGB_DIM: usize = 3; // rgb color dimension
pub const ITER_NUM: u32 = 250;
pub const BLACK_COLOR: Rgb<u8> = Rgb { data: [0 as u8; RGB_DIM] };

/// Try to determine if `c` is in the Mandelbrot set, using at most `limit`
/// iterations to decide.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius two centered on the
/// origin. If `c` seems to be a member (more precisely, if we reached the
/// iteration limit without being able to prove that `c` is not a member),
/// return `None`.

fn escape_time(c: Complex<f64>) -> (Complex<f64>, Option<u32>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..ITER_NUM {
        z = z * z + c;
        if z.norm_sqr().sqrt() >= R {
            return (z, Some(i));
        }
    }
    (z, None)
}

/// get rgb color
/// From: https://www.reddit.com/r/math/comments/2abwyt/smooth_colour_mandelbrot/

pub fn get_color_rgb(c: Complex<f64>) -> Rgb<u8> {
    let (z, count) = escape_time(c);
    let data: [f64; RGB_DIM] = match count {
        None => [0.0; RGB_DIM],
        Some(i) => {
            let mut v = (i as f64 + R - z.norm_sqr().sqrt().log2().log2()).log2() / 5.0;
            if v < 1.0 {
                // v**4, v**2.5, v
                [v.powi(4), v.powf(2.5), v]
            } else {
                // v = max(0, 2 - v); v, v**1.5, v**3
                v = (2.0 - v).max(0.0);
                [v, v.powf(1.5), v.powf(3.0)]
            }
        }
    };

    let mut color_data = [0 as u8; RGB_DIM];
    for (i, item) in data.iter().enumerate() {
        color_data[i] = (item * 255.0) as u8;
    }
    Rgb { data: color_data }
}
