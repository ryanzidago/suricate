use clap::Parser;
use notify::Event;
use notify::RecursiveMode;
use notify::Watcher;
use std::path::Path;
use std::process::Command;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path to watch for changes
    #[arg(short, long)]
    path: String,

    /// The file extensions to watch for changes
    #[arg(short, long, value_parser, num_args = 0.., value_delimiter = ',')]
    extensions: Vec<String>,

    /// The commands to execute when a file matching the path and extension changes
    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ',')]
    commands: Vec<String>,
}

struct CommandState {
    is_running: bool,
}

fn main() {
    let args = Args::parse();
    let (tx, rx) = mpsc::channel::<Result<Event, notify::Error>>();
    let mut watcher = notify::recommended_watcher(tx).expect("Could not create file watcher");
    watcher
        .watch(Path::new(&args.path), RecursiveMode::Recursive)
        .expect("Could not watch directory");

    println!("Suricate starting ...");
    println!("Watching directory: {}\n", args.path);

    let command_state = Arc::new(Mutex::new(CommandState { is_running: false }));

    for res in rx {
        match res {
            Ok(event) => handle_event(event, &args, command_state.clone()),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
}

fn handle_event(event: Event, args: &Args, command_state: Arc<Mutex<CommandState>>) {
    if event.kind.is_modify() {
        if let Some(saved_path) = event
            .paths
            .iter()
            .find(|path| is_relevant_file(path, &args.extensions))
        {
            // Check if command is already running
            let mut state = command_state.lock().unwrap();
            if !state.is_running {
                state.is_running = true;

                println!("File {:?} has been modified", saved_path);

                let command_state_clone = Arc::clone(&command_state);
                let commands = args.commands.clone();
                let path = args.path.to_owned();

                thread::spawn(move || {
                    let path = Path::new(&path);
                    execute_commands(commands, path);
                    let mut state = command_state_clone.lock().unwrap();
                    state.is_running = false;
                });
            }
        }
    }
}

fn is_relevant_file(path: &Path, extensions: &Vec<String>) -> bool {
    if extensions.is_empty() {
        return true;
    }

    if let Some(extension) = path.extension() {
        let extension = extension.to_string_lossy().to_lowercase();
        extensions.iter().any(|ext| ext == &extension)
    } else {
        false
    }
}

fn execute_commands(commands: Vec<String>, path: &Path) {
    let commands = commands
        .into_iter()
        .map(|cmd| cmd.trim().to_string())
        .collect::<Vec<String>>();

    for command in commands {
        let parts = command.splitn(2, ' ').collect::<Vec<&str>>();
        let (cmd, args) = if parts.len() == 2 {
            (parts[0], parts[1])
        } else {
            (parts[0], "")
        };

        let mut cmd = Command::new(cmd);

        if args.is_empty() {
            println!("Running {:?}", cmd);
        } else {
            println!("Running {:?} with args {:?}", cmd, args);
            cmd.args(args.split_whitespace());
        }

        cmd.current_dir(&path)
            .status()
            .expect("Command execution failed");
    }

    println!("");
}
