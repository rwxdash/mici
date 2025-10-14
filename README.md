# mici

`mici` is a lightweight CLI framework that automatically discovers and executes your commands based on filesystem hierarchy.

Define your commands as YAML files and `mici` handles the CLI and its execution for you.

## Quick Start

Make sure `Rust` is installed. See [#install](#install) for more details.

```bash
# Install mici using Cargo
# This will install `mici` and `mici` as executables.
cargo install mici
```

Run `mici --help` or `mici --help` to see what's available.

```bash
# Initialize mici
mici init

# Create your first command and edit if needed
# at `~/.mici/jobs/commands/hello.yml`
mici new hello

# See what it is with --help
# Pager can be disabled with `disable_pager: true` in the `config.yml`.
mici hello --help

# Run it
mici hello
```

## Why mici?

**Traditional CLI development:**
- Write command parsers and argument handling
- Manage command registration and routing
- Rebuild and redeploy for every new command
- Maintain complex CLI application code

**With mici:**
- Drop YAML files in a directory structure
- Commands appear in your CLI automatically
- No rebuilds, no deployments, no CLI code
- Perfect for CI/CD replacement workflows

## How it works

Create your commands as YAML files in a directory hierarchy.

```
~/.mici
├── config.yml                          # Configuration file
└── jobs
    ├── commands
    │   ├── deploy
    │   │   ├── terraform.yml           # mici deploy terraform
    │   │   └── frontend
    │   │       ├── staging.yml         # mici deploy frontend staging
    │   │       └── production.yml      # mici deploy frontend production
    │   ├── database
    │   │   ├── backup.yml              # mici database backup
    │   │   └── migrate.yml             # mici database migrate
    │   └── hello.yml                   # mici hello
    │
    └── scripts                         # Usable as:
        ├── hello.py                    #   scripts/hello.py
        └── run.sh                      #   scripts/run.sh
```

Each YAML file has CI-like attributes - environment variables, confirmation prompts, parallel execution, and more; allowing `mici` to customize your run of that command and generate `help` documentation based on the available information.

```yaml
version: "1.0"
name: "deploy staging"
description: "Deploy to staging environment"
configuration:
  confirm: true
  environment:
    DEPLOY_ENV: "staging"
    API_KEY: "${STAGING_API_KEY}"
steps:
  - id: "build"
    name: "Build application"
    run:
      shell: "bash"
      command: "npm run build"
  - id: "deploy"
    name: "Deploy application"
    run:
      shell: "bash"
      command: "kubectl apply -f k8s/staging/"
```

That's it. Your filesystem **is** your CLI structure, and YAML **is** your configuration.

## Install

`Cargo` is a package manager for `Rust`. Make sure to have Rust toolset available on your computer first. See [`rustup` installation guide](https://www.rust-lang.org/tools/install) for easy introduction.

Once you have `Rust` available, you can run any of the following commands to install `mici`.

### From crates.io

```bash
cargo install mici
```

### From source

```bash
git clone git@github.com:rwxdash/mici.git
cd ./mici

cargo install --path .
```

## Uninstall

Simply run:

```bash
cargo uninstall mici
```

## What's to come

There are some major stories to complete before I call this project version 1.0. Here's what I have in my mind so far:

#### TODOs

- [] Implement tests and CI checks
- [x] Implement the basic runner
    + [x] Handle step confirmation
    + [x] Basic execution of a simple command
    + [x] Implement environment variable substitution
    + [x] Make basic execution run on Windows
    + [] Implement `script: "scripts/..."` usage for steps
- [] Better logging throughout the program
    + [] Error/Debug log level can be set through config file
    + [] Can be set to None/Silent
    + [] Implement tracing
      + [] Add tracing for rust/otel    # https://github.com/tokio-rs/tracing
- [x] The default schema in new command should have proper default shell for linux/windows
- [] Implement expression evaluator for `when:` in steps
    + [] on_failure()                  # any previous step failed
    + [] on_success()                  # all previous steps passed
    + [] on_platform("linux")          # linux/win/darwin
    + [] depends_on("step_id")         # or depends_on(["step1", "step2"])
    + [] ${ENV_VAR} == "production"
    + [] @{inputs.cleanup}
    + [] @{inputs.branch} == "main"
    + [] @{steps.<STEP_ID>.output} == "success" (Or shell code? not sure)
    + [] Accept operators and chains
    + [] Implement variable masking (?)
- [x] Basic validation for commands
- [x] Handle errors with miette          # https://github.com/zkat/miette
    + [x] Might need a new error class
- [] Think about runner/step-execution isolation
    + with chroot/containers/microvms

## Contributions and Code of Conduct

**Code of conduct** is simple. Be nice and thoughtful. That's all.

About the **Contributions**, the project is obviously open to contributions. However, since it's quite early and I'm still shaping what this tool can be, no major refactor or feature PRs, please.

I'd very much appreciate if you open an issue beforehand if you plan to contribute any change.

---

*mici: Because your filesystem is the best argument parser.*
