use std::env;
use std::path;
// use std::process;

extern crate getopts;
use getopts::Options;

mod utils;

static PROJECT_DIR: &str = "~/.minici";

// fn check_help()

fn main() {
    println!(
        "project dir exists? : {}",
        path::Path::new(PROJECT_DIR).exists()
    );

    utils::fs::check_project(PROJECT_DIR);

    let args: Vec<String> = env::args().collect();
    // let program = args[0].clone();
    let mut opts = Options::new();
    {
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

    // println!("{:?}", &args);
    // println!("{:?}", &program);

    match &args.get(1).map(String::as_ref) {
        Some("init") => {
            // run init
            opts.optflag("", "clean", "Do a clean setup for ~/.minici");
            let matches = match opts.parse(&args[1..]) {
                Ok(m) => m,
                Err(_) => return,
            };
            if matches.opt_present("clean") {
                // clean setup
            } else {
                // check if it's already present. if so, ignore
            }
        }
        Some("seed") => {
            // run seed
            let mut branch = String::new();
            opts.optopt(
                "b",
                "branch",
                "The branch where we populate the seeds from",
                "-b BRANCH_NAME",
            );
            let matches = match opts.parse(&args[1..]) {
                Ok(m) => m,
                Err(_) => return,
            };
            if matches.opt_present("b") {
                println!("{:?}", matches.opt_positions("b"));
                branch = matches.opt_str("b").unwrap();
            }

            println!("{}", branch);
        }
        Some("update") => {
            // run update
        }
        Some("version") => {
            // print version
            println!("version 0.1.0");
        }
        Some("help") => {
            // print help
            println!("help");
        }
        Some(_) => {
            // check command
        }
        None => {
            println!("help none");
        }
    }
}

// minici root project subcommand [flag]

// minici   thundra         foresight       frontend        deploy
//          maintenance     slack-bot       update          --anyflag="123"

/*

> check for help and version flag
> check for other flags
> pop flags
> joint the rest with `/`
> validate file is present
    - error if not
    - print help
> parse the command yaml
> run the steps

*/
