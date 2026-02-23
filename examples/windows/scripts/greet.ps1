# This script is called by mici via: mici greet --name "World"
# Inputs are available as MICI_INPUT_* environment variables.

$name = if ($env:MICI_INPUT_NAME) { $env:MICI_INPUT_NAME } else { "stranger" }
$force = if ($env:MICI_INPUT_FORCE) { $env:MICI_INPUT_FORCE } else { "false" }

Write-Output "Hello, $name!"
Write-Output "Force mode: $force"
