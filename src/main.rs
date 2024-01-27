use ariadne::prelude::*;
use std::ffi::OsString;
use std::{
    error::Error,
    fs,
    io::Write,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use clap::{Parser, Subcommand};
use rustyline::error::ReadlineError;
use rustyline::Editor;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create a maze
    Create {
        width: usize,
        height: usize,
        #[clap(short, long)]
        visualize: Option<bool>,
    },
    /// Solve a maze
    Solve {
        #[clap(short, long)]
        visualize: Option<bool>,
    },
    /// Load a maze from a JSON file
    Load { file_path: PathBuf },
    /// Export a maze to a JSON file
    Export { file_path: PathBuf },
    /// Clear the prompt window
    Clear,
    /// Show the current maze
    Show,
    /// Quit the program
    Quit,
    /// Quit the program
    Exit,
}

mod commands;

fn string_to_args(string: &str) -> Vec<OsString> {
    let mut args = vec![OsString::from("ariadne")];

    for arg in string.split_whitespace() {
        args.push(arg.into());
    }

    args
}

fn main() -> Result<(), Box<dyn Error>> {
    let home = std::env::var("HOME").unwrap();
    let path = format!("{}/.ariadne_history", home);

    let mut maze: Arc<RwLock<Maze>> = Arc::new(RwLock::new(Maze::new(10, 10)));
    let mut rl = Editor::<()>::new()?;
    if rl.load_history(&path).is_err() {
        fs::File::create(&path)?;
    }

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match Cli::try_parse_from(string_to_args(&line)) {
                    Ok(cli) => match cli.command {
                        Commands::Quit | Commands::Exit => commands::quit::command(&home, &mut rl)?,
                        Commands::Show => println!("{}", maze.read().unwrap().string()),
                        Commands::Load { file_path } => {
                            let path = fs::canonicalize(file_path)?;
                            maze = Arc::new(RwLock::new(Maze::from(path)));
                        }
                        Commands::Export { file_path } => {
                            let mut path = std::env::current_dir()?;
                            path.push(file_path);
                            let mut file = fs::File::create(path)?;
                            let s = serde_json::to_string_pretty(&maze.read().unwrap().clone())?;
                            write!(file, "{}", s)?;
                        }
                        Commands::Clear => commands::clear::command()?,
                        Commands::Solve { visualize } => {
                            commands::solve::command(Arc::clone(&maze), visualize)?
                        }
                        Commands::Create {
                            width,
                            height,
                            visualize,
                        } => maze = commands::create::command(visualize, width, height)?,
                    },
                    Err(e) => println!("{}", e),
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Unexpected error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(&path)?;

    Ok(())
}
