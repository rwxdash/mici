extern crate colored;

use crate::lib::maintenance::base_command::BaseCommand;
use crate::utils::fs::project_folder;
use std::error::Error;
use std::path::Path;
use std::process;

#[allow(dead_code)]
pub struct SeedCommand {
    pub base: BaseCommand,
}

impl SeedCommand {
    pub const fn new() -> Self {
        Self {
            base: BaseCommand {
                name: "minici seed",
                description: "Used for populating the commands",
                synopsis: "
    minici seed
        [-b, --branch <value>]
                ",
                options: "
    -b, --branch (string)
        Will take a string as branch to checkout and
        populate the commands from.
                ",
                usage: "minici seed [-b, --branch <value>]",
            },
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let minici_exist = Path::new(&project_folder()).exists();
        if !minici_exist {
            // print err and exit
            process::exit(1)
        }

        // read ~/.minici/config.yml
        // if config.yml is missing, prompt to get repo url
        // - exit if not given
        // write the repo to config.yml
        // read config yml
        // clone repo to /tmp/HASH
        // pushd repo
        // check if `~/.minici/jobs` is present, if not create dir
        // diff ./seeds to ~/.minici/jobs
        // cp ./seeds to ~/.minici/jobs
        // popd repo
        // rm repo

        return Ok(());
    }
}

pub const SEED_COMMAND: SeedCommand = SeedCommand::new();
