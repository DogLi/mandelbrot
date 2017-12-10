use clap;

pub fn get_args() -> clap::ArgMatches<'static> {
    clap::App::new("mandelbrot")
        //.setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .version(crate_version!())
        .author(crate_authors!())
        .about("tools to make colord mandelbrot set")
        .arg(clap::Arg::with_name("pixels")
            .short("p")
            .long("pixels")
            .value_name("pixels")
            .takes_value(true)
            .required(true)
            .default_value("1200x960")
            .help("set the picture pixels!"))
        .arg(clap::Arg::with_name("begin_point")
            .short("b")
            .long("begin_point")
            .required(true)
            .takes_value(true)
            .default_value("-1.20,0.35")
            .help("set the begin complex value which re and im are separated by ',' "))
        .arg(clap::Arg::with_name("end_point")
            .short("e")
            .long("end_point")
            .required(true)
            .takes_value(true)
            .default_value("-1,0.20")
            .help("set the end complex value which re and im are separated by ',' "))
        .arg(clap::Arg::with_name("file")
            .short("f")
            .long("file")
            .required(true)
            .takes_value(true)
            .help("set the file path which should ends with .png!"))
        .get_matches()
}