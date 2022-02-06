extern crate colored;

use crate::lib::maintenance::base_command::BaseCommand;
use crate::lib::maintenance::base_command::InitConfiguration;
use crate::utils::fs::{create_tmp_folder, get_config_file, get_home_dir, get_project_folder};
use git2::{Cred, RemoteCallbacks};
use std::error::Error;
use std::path::Path;
use std::process;

pub struct SeedCommand {
    pub base: BaseCommand,
}

impl SeedCommand {
    pub const fn new() -> Self {
        Self {
            base: BaseCommand {
                name: "minici seed",
                description: "Used for populating the commands",
                synopsis: "minici seed [options]",
                options: "
    -b, --branch (string)
        Will take a string as branch to checkout and
        populate the commands from.
                ",
                usage: "
    minici seed
        [-b, --branch <value>]
                ",
            },
        }
    }

    pub fn run(&self, branch: Option<String>) -> Result<(), Box<dyn Error>> {
        let minici_exist = Path::new(&get_project_folder()).exists();
        if !minici_exist {
            // TODO: print err and exit
            println!("> Exiting...");
            process::exit(1)
        }

        let config_exist = Path::new(&get_config_file()).exists();
        if !config_exist {
            // TODO: print err and exit
            println!("> Exiting...");
            process::exit(1)
        }

        let config_file = std::fs::read_to_string(Path::new(&get_config_file())).unwrap();
        let init_configuration: InitConfiguration = serde_yaml::from_str(&config_file)?;

        let tmp_folder = create_tmp_folder();

        println!("{}", tmp_folder);

        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key(
                username_from_url.unwrap(),
                None,
                std::path::Path::new(&format!("{}/.ssh/id_rsa", &get_home_dir())),
                None,
            )
        });

        // Prepare fetch options.
        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(callbacks);

        // Prepare builder.
        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fo);

        if branch.is_some() {
            builder.branch(&branch.unwrap());
        }

        // Clone the project.
        builder
            .clone(
                &init_configuration.upstream_url.as_str(),
                Path::new(&tmp_folder),
            )
            .expect("Failed to clone the repository");

        return Ok(());
    }
}

pub const SEED_COMMAND: SeedCommand = SeedCommand::new();
