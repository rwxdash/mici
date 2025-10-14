extern crate colored;

use crate::cli::core::base_command::BaseCommand;
use crate::cli::core::base_command::InitConfiguration;
use crate::utils::fs::{
    clear_jobs_folder, copy_directory, create_tmp_folder, get_config_file, get_jobs_folder,
    get_project_folder,
};
use git2::{Cred, CredentialType, RemoteCallbacks};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process;

pub struct FetchCommand {
    pub base: BaseCommand,
}

impl FetchCommand {
    pub const fn new() -> Self {
        Self {
            base: BaseCommand {
                name: "mici fetch",
                description: "
    Synchronizes and updates local commands by cloning from a specified remote
    repository.
                ",
                synopsis: "mici fetch [options]",
                options: "
    -b, --branch <name>     (option)
    Specify the branch to fetch and use for updating local commands.
    Defaults to the repository's default branch if not provided.
                ",
                usage: "
    mici fetch           # Fetches default branch from remote
    mici fetch -b dev    # Fetches `dev` branch from remote
                ",
            },
        }
    }

    pub fn run(&self, branch: Option<String>) -> Result<(), Box<dyn Error>> {
        // TODO: Warn that this cant be undone!
        // TODO: Better logging

        let mici_exist = Path::new(&get_project_folder()).exists();
        if !mici_exist {
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
        let init_configuration: InitConfiguration = serde_yaml::from_str(&config_file).unwrap();

        if init_configuration.upstream_url.is_none() {
            // TODO: print err and exit
            println!("> Exiting...");
            process::exit(1)
        }

        let tmp_folder = create_tmp_folder();

        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, allowed_types| {
            let username = username_from_url.unwrap_or("git");

            if allowed_types.contains(CredentialType::SSH_KEY) {
                return Cred::ssh_key_from_agent(username);
            }

            if allowed_types.contains(CredentialType::USER_PASS_PLAINTEXT) {
                let token_vars = ["GITHUB_TOKEN", "GITLAB_TOKEN", "GIT_TOKEN"];

                for var in &token_vars {
                    if let Ok(token) = std::env::var(var) {
                        return Cred::userpass_plaintext(username, &token);
                    }
                }
            }

            Err(git2::Error::from_str("No authentication method available"))
        });

        let mut fetch_options = git2::FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);
        fetch_options.depth(1);

        let mut builder = git2::build::RepoBuilder::new();

        builder.fetch_options(fetch_options);

        if branch.is_some() {
            builder.branch(&branch.unwrap());
        }

        builder
            .clone(
                &init_configuration.upstream_url.unwrap().as_str(),
                Path::new(&tmp_folder),
            )
            .expect("Failed to clone the repository");

        clear_jobs_folder().expect("Failed to clear the jobs directory");

        copy_directory(
            Path::new(&tmp_folder)
                .join(init_configuration.upstream_cmd_path.unwrap())
                .to_str()
                .unwrap(),
            &get_jobs_folder(),
        )
        .expect("Failed to copy upstream to the jobs directory");

        fs::remove_dir_all(Path::new(&get_jobs_folder()).join(".git"))
            .expect("Failed to remove .git directory");
        fs::remove_dir_all(&tmp_folder).expect("Failed to remove temporary folder");

        return Ok(());
    }
}

pub const FETCH_COMMAND: FetchCommand = FetchCommand::new();
