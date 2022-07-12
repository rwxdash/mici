The idea is that users of this CLI should manage their commands, YAMLs and such, in a separate Git repository. When you first setup the application and run the initializer, it will ask about this repository and the certain path that the commands resides in that repository. Then it'll write the configuration to the users `$HOME` path, specifically at `~/.minici/config.yml`.

```yaml
upstream_url: "repo.url"        # can be SSH or HTTPS
upstream_cmd_path: "./commands"   # an example path
```
After setting this up, user then run the seed command to populate, ie. copy the commands from the repository and path pair from the configuration file, into `~/.minici/commands/*`. Any **folder** in this folder will be assumed as a subcommand to this CLI and any YAML file can be callable. To access a command in a folder, the path to the YAML file should be given using spaces.

For example, in the following folder structure;

```text
commands
├── arbitrary-command.yml
└── my-application
    ├── frontend
    │   └── deploy.yml
    └── backend
        └── deploy.yml
```

the callable commands from the CLI would be;

```bash
minici arbitrary-command              [options]
minici my-application frontend deploy [options]
minici my-application backend deploy  [options]
```
