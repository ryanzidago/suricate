# Suricate 

CLI tool that executes speficied commands when a file changes matches a specific directory and extensions.

Usage: suricate [OPTIONS]

Options:
-  -p, --path <PATH>                   The path to watch for changes
-   -e, --extensions [<EXTENSIONS>...] A comma-separated list of file extensions to watch for changes
-  -c, --commands <COMMANDS>...        A comma-separated list of commands to execute when a file matching the path and extension changes
-  -h, --help                          Print help
-  -V, --version                       Print version

Examples:
```bash
suricate --path path/to/my/elixir/phoenix/live_view/app --extension='ex,exs,heex' --commands='mix format, mix compile'
```

```bash
suricate -p my_app -e='ex, exs, heex' -c='mix compile, mix test'
```
