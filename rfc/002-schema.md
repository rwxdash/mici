An example schema for a command.

Haven't thought how to go with all this features yet. There are parts that I still haven't figured it out, but it looks like a good start.

#### Schema

```yaml
description: Description
usage: |
  USAGE_DETAILS
configuration:
  # expect confirmation from the user
  confirm: true
  # run steps asyncronously and independently, default false
  async: true | false
  # env vars for this run
  environment:
    KEY: VALUE
  #! options to take from terminal with `--` prefix, `-` if value is one letter (?)
  #! value can be accessed inside the YAML via {{ options.<key> }}
  options:
    - branch
  #! authorization group
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
