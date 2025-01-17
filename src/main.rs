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
    #[arg(long)]
    watched_path: String,
}

struct CommandState {
    is_running: bool,
}

fn main() {
    let args = Args::parse();
    let (tx, rx) = mpsc::channel::<Result<Event, notify::Error>>();
    let mut watcher = notify::recommended_watcher(tx).expect("Could not create file watcher");
    watcher
        .watch(Path::new(&args.watched_path), RecursiveMode::Recursive)
        .expect("Could not watch directory");

    println!("Suricate starting ...");
    println!("Watching directory: {}\n", args.watched_path);

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
        if let Some(saved_path) = event.paths.iter().find(|path| is_elixir_file(path)) {
            // Check if command is already running
            let mut state = command_state.lock().unwrap();
            if !state.is_running {
                state.is_running = true;

                println!("File {:?} has been modified", saved_path);
                let command_state_clone = Arc::clone(&command_state);
                let watched_path = args.watched_path.to_owned();
                let saved_path = saved_path.to_string_lossy().into_owned();

                thread::spawn(move || {
                    let watched_path = Path::new(&watched_path);
                    let saved_path = Path::new(&saved_path);
                    execute_commands(watched_path, saved_path);
                    let mut state = command_state_clone.lock().unwrap();
                    state.is_running = false;
                });
            }
        }
    }
}

fn is_elixir_file(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        matches!(
            extension.to_str().expect("Could not read file extension"),
            "ex" | "exs" | "heex"
        )
    } else {
        false
    }
}
fn execute_commands(watched_path: &Path, saved_path: &Path) {
    Command::new("mix")
        .arg("format")
        .arg(&saved_path)
        .current_dir(&watched_path)
        .status()
        .expect("Could not format file");

    Command::new("mix")
        .arg("compile")
        .current_dir(&watched_path)
        .status()
        .expect("Could not compile project");

    println!("");
}
