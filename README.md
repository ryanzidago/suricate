# Suricate

CLI tool that executes specified commands when a file changes matches a specific directory and extensions.

## Installation

Clone this repository and then run `cargo build --release`.  You now have the executable in `./target/release/suricate`.

## Usage

Usage: suricate [OPTIONS]

Options:
-  -p, --path <PATH>                   The path to watch for changes
-  -e, --extensions [<EXTENSIONS>...]  A comma-separated list of file extensions to watch for changes
-  -c, --commands <COMMANDS>...        A comma-separated list of commands to execute when a file matching the path and extension changes
-  -h, --help                          Print help
-  -V, --version                       Print version

Examples:
```bash
suricate --path path/to/my/phoenix/app --extensions='ex,exs,heex' --commands='mix format, mix compile, MIX_ENV=test mix compile'
```

```bash
suricate -p my_app -e='ex, exs, heex' -c='mix compile, mix test'
```