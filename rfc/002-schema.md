An example schema for a command.

Haven't thought how to go with all this features yet. There are parts that I still haven't figured it out, but it looks like a good start.

#### Schema

```yaml
# schema version
version: 1
description: Description
usage: |
  USAGE_DETAILS
configuration:
  # expect confirmation from the user, default false
  confirm: true | false
  # run steps in parallel, default false
  parallel: true | false
  # env vars for this run
  environment:
    KEY: VALUE
  # value can be accessed inside the YAML via {{ options.<key> }}
  options:
    - long: long_name         # eg. --branch
      short: short_name       # eg. -b
      required: true | false  # default false
      flag: true | false      # flag or option. default false.
      default: main           # default value when flag is unpresent
      description: Repository branch to clone
  group:
    - developer
    - devops
    - admin
steps:
  - name: Step 1
    run:
      # Default: /usr/bin/bash
      # Alternate: /usr/bin/python3 or any other executable
      shell: /usr/bin/bash
      # Env var for this step only
      environment:
        KEY: VALUE
      command: echo "hello"
  - name: Step 2
    run:
      # run even if previous steps are failed
      always: true
      # ...
```
