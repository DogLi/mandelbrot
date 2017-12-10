use super::{parser, color};
use num::Complex;


#[test]
fn test_color() {
    let point = Complex { re: -2.5, im: 1.2};
    let (z, count) = escape_time(point);
    assert_eq!(count, Some(1));
    let rgb = get_color_rgb(z, count);
    assert_eq!(rgb.data, [5, 22, 96]);
}

#[test]
fn test_parser_pair() {
    assert_eq!(parser::parse_pair::<i32>("", ','), None);
    assert_eq!(parser::parse_pair::<i32>("10,", ','), None);
    assert_eq!(parser::parse_pair::<i32>(",10", ','), None);
    assert_eq!(parser::parse_pair::<i32>("5,10", ','), Some((5, 10)));
    assert_eq!(parser::parse_pair::<i32>("5,10ab", ','), None);
    assert_eq!(parser::parse_pair::<f64>("0.5x", ','), None);
    assert_eq!(parser::parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

#[test]
fn test_parser_complex() {
    assert_eq!(
        parser::parse_complex("1.25,-0.025"),
        Some(Complex {
            re: 1.25,
            im: -0.025,
        })
    );
    assert_eq!(parser::parse_complex(",-0,1"), None);
}

#[test]
fn test_parser_pixel2point() {
    assert_eq!(
        prser::pixel2point(
            (100, 100),
            (25, 75),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 },
        ),
        Complex { re: -0.5, im: -0.5 }
    )
}
