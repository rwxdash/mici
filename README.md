# minici

`minici` (or `mci`) is a lightweight CLI framework that automatically discovers and executes your commands based on filesystem hierarchy.

Define your commands as YAML files and `minici` handles the CLI interface and execution for you.

## Quick Start

Make sure `Rust` is installed. See [#install](#install) for more details.

```bash
# Install minici using Cargo
# This will install `minici` and `mci` as executables.
cargo install minici
```

Run `minici --help` or `mci --help` to see what's available.

```bash
# Initialize minici
mci init

# Create your first command and edit if needed
# at `~/.minici/jobs/commands/hello.yml`
mci new hello

# See what it is with --help
# Pager can be disabled with `disable_pager: true` in the `config.yml`.
mci hello --help

# Run it
mci hello
```

## Why minici?

❌ **Traditional CLI development:**
- Write command parsers and argument handling
- Manage command registration and routing
- Rebuild and redeploy for every new command
- Maintain complex CLI application code

✅ **With minici:**
- Drop YAML files in a directory structure
- Commands appear in your CLI automatically
- No rebuilds, no deployments, no CLI code
- Perfect for CI/CD replacement workflows

## How it works

```
~/.minici
├── config.yml                         # Configuration file
└── jobs
    ├── commands
    │   ├── deploy
    │   │   ├── terraform.yml          # mci deploy terraform
    │   │   └── frontend
    │   │       ├── staging.yml        # mci deploy frontend staging
    │   │       └── production.yml     # mci deploy frontend production
    │   ├── database
    │   │   ├── backup.yml             # mci database backup
    │   │   └── migrate.yml            # mci database migrate
    │   └── hello.yml                  # mci hello
    │
    └── scripts                        # Importable as (TO_BE_DECIDED)
        ├── hello.py                   # ${scripts/hello.py}
        └── run.sh                     # ${scripts/run.sh}
```

Each command is a YAML file with CI-like attributes - environment variables, confirmation prompts, parallel execution, and more.

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
  - name: "build"
    run:
      shell: "bash"
      command: "npm run build"
  - name: "deploy"
    run:
      shell: "bash"
      command: "kubectl apply -f k8s/staging/"
```

That's it. Your filesystem **is** your CLI structure, and YAML **is** your configuration.

## Use Cases

### 🔄 **CI/CD Replacement**
Replace complex CI/CD pipelines with simple scripts that minici can execute locally or remotely.

### 🛠️ **Development Workflows**
Organize project-specific commands without cluttering your global CLI or maintaining custom tooling.

### 🏢 **Team Automation**
Share executable workflows through Git—everyone gets the same commands automatically.

### 📦 **Microservice Management**
Each service can have its own command directory that integrates seamlessly into a unified CLI.

Commands you create appear automatically - no registration needed.

## Install

`Cargo` is a package manager for `Rust`. Make sure to have Rust toolset available on your computer first. See [`rustup` installation guide](https://www.rust-lang.org/tools/install) for easy introduction.

Once you have `Rust` available, you can run any of the following commands to install `minici`.

### From crates.io

```bash
cargo install minici
```

### From source

```bash
git clone git@github.com:rwxdash/minici.git && cd $_

cargo install --path .
```

## Uninstall

Simply run:

```bash
cargo uninstall minici
```

## Documentation

> TODO

## Contribution

> TODO

## License

> TODO

---

*minici: Because your filesystem is the best argument parser.*
