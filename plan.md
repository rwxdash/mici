# plan

- read repository information from ~/.minici/config.yml

---

### MAINTENANCE

- `minici init [--clean]`
    - ask for the repository url (ssh)
    - create ~/.minici
    - write ~/.minici/config.yml
- `minici seed [-b | --branch xxx]`
    - fetch repo to tmp
    - checkout to branch
    - print diff between ./seeds and ~/.minici/jobs/
    - get confirm (?)
    - replace everything in ~/.minici
- `minici update`
    - check project exist
        - if not run minici init
    - fetch repo to tmp
    - install cargo
    - seed commands from repository
- `minici version`
- `minici help`

### COMMANDS
- `minici thundra ...`
