extern crate mandelbrot;
#[macro_use]
extern crate clap;

mod args;
use mandelbrot::{parser, run};

fn main() {
    let matches = args::get_args();
    let bounds = matches.value_of("pixels").unwrap();
    let begin_point = matches.value_of("begin_point").unwrap();
    let end_point = matches.value_of("end_point").unwrap();
    let file = matches.value_of("file").unwrap();

    let bounds = parser::parse_pair::<usize>(bounds, 'x').expect("error parsing image dimensions");
    let begin_point = parser::parse_complex(begin_point).expect("error parsing left corner point");
    let end_point = parser::parse_complex(end_point).expect("error parsing left corner point");

    run(bounds, begin_point, end_point, file);
}
