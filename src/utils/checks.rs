use getopts::Options;

pub fn check_help_and_version(args: &Vec<String>) {
    let mut opts = Options::new();

    opts.optflag("h", "help", "");
    opts.optflag("v", "version", "");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_) => return,
    };

    if matches.opt_present("version") {
        println!("version 1");
    }

    if matches.opt_present("help") {
        println!("caught help at {:?}", matches.opt_positions("help"));
        return;
    }
}
