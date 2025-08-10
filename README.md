# minici

`minici` is a lightweight CLI framework that automatically discovers and executes your commands based on filesystem hierarchy.

Define your commands as YAML files and `minici` handles the CLI interface and execution for you.

## Quick Start

Run `minici --help` to view what's available.

```bash
# Initialize minici
minici init

# Create your first command and edit if needed
# at `~/.minici/jobs/commands/hello.yml`
minici new hello

# See what it is with --help
# Pager can be disabled with `NOPAGER=1` environment variable.
minici hello --help

# Run it
minici hello
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
    │   │   ├── terraform.yml          # minici deploy terraform
    │   │   └── frontend
    │   │       ├── staging.yml        # minici deploy frontend staging
    │   │       └── production.yml     # minici deploy frontend production
    │   ├── database
    │   │   ├── backup.yml             # minici database backup
    │   │   └── migrate.yml            # minici database migrate
    │   └── hello.yml                  # minici hello
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
