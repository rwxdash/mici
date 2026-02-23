#!/bin/bash
# This script is called by mici via: mici greet --name "World"
# Inputs are available as MICI_INPUT_* environment variables.

echo "Hello, ${MICI_INPUT_NAME:-stranger}!"
echo "Force mode: ${MICI_INPUT_FORCE:-false}"
