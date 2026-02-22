use crate::cli::core::base_command::BaseCommand;
use crate::cli::core::base_command::InitConfiguration;
use crate::utils::fs::{
    clear_jobs_folder, copy_directory, create_tmp_folder, get_config_file, get_jobs_folder,
    get_project_folder,
};
use dialoguer::{Confirm, theme::ColorfulTheme};
use git2::{Cred, CredentialType, RemoteCallbacks};
use std::error::Error;
use std::fs;
use std::io::IsTerminal;

pub struct FetchCommand {
    pub base: BaseCommand,
}

impl Default for FetchCommand {
    fn default() -> Self {
        Self::new()
    }
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

    -f, --force             (flag)
    Skip the confirmation prompt. Useful for CI/scripts.
                ",
                usage: "
    mici fetch           # Fetches default branch from remote
    mici fetch -b dev    # Fetches `dev` branch from remote
    mici fetch --force   # Fetches without confirmation
                ",
            },
        }
    }

    pub fn run(&self, branch: Option<String>, force: bool) -> Result<(), Box<dyn Error>> {
        let project_folder = get_project_folder();
        if !project_folder.exists() {
            return Err("mici is not initialized. Run 'mici init' first.".into());
        }

        let config_file = get_config_file();
        if !config_file.exists() {
            return Err("Configuration file not found. Run 'mici init' first.".into());
        }

        let config_content = std::fs::read_to_string(&config_file)?;
        let init_configuration: InitConfiguration = serde_yaml::from_str(&config_content)?;

        let upstream_url = init_configuration
            .upstream_url
            .ok_or("No upstream URL configured. Run 'mici init' to set one.")?;

        if force {
            println!(
                "> Confirmation skipped with --force. This operation is destructive and will replace all local commands."
            );
        } else {
            println!("> This will replace all local commands with the upstream version.");
            println!("> Use --force to skip this confirmation.");

            let confirmed = if std::io::stdin().is_terminal() {
                Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Do you want to continue?")
                    .default(false)
                    .interact()?
            } else {
                false
            };

            if !confirmed {
                println!("> Fetch cancelled.");
                return Ok(());
            }
        }

        let tmp_folder = create_tmp_folder()?;

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

        if let Some(ref b) = branch {
            builder.branch(b);
        }

        tracing::info!("Cloning from {}", &upstream_url);

        builder
            .clone(&upstream_url, &tmp_folder)
            .map_err(|e| format!("Failed to clone the repository: {}", e))?;

        clear_jobs_folder().map_err(|e| format!("Failed to clear the jobs directory: {}", e))?;

        let upstream_cmd_path = init_configuration
            .upstream_cmd_path
            .ok_or("No upstream command path configured.")?;

        copy_directory(&tmp_folder.join(&upstream_cmd_path), &get_jobs_folder())
            .map_err(|e| format!("Failed to copy upstream to the jobs directory: {}", e))?;

        let git_dir = get_jobs_folder().join(".git");
        if git_dir.exists() {
            fs::remove_dir_all(&git_dir)
                .map_err(|e| format!("Failed to remove .git directory: {}", e))?;
        }
        fs::remove_dir_all(&tmp_folder)
            .map_err(|e| format!("Failed to remove temporary folder: {}", e))?;

        tracing::info!("Fetch complete");

        Ok(())
    }
}

pub const FETCH_COMMAND: FetchCommand = FetchCommand::new();
