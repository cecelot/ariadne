use self::lib::{coordinate::Coordinates, prelude::*};
use std::ffi::OsString;
use std::{
  collections::HashSet,
  error::Error,
  fs,
  io::{stdin, stdout, Write},
  path::PathBuf,
  sync::{mpsc, Arc, RwLock},
  thread,
  time::Duration,
};

use clap::{Parser, Subcommand};
use colored::Colorize;

use crossterm::{
  cursor::{Hide, MoveTo, Show},
  event::{read, DisableMouseCapture},
  execute,
  terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetTitle},
};

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

mod lib;

fn string_to_args(string: &str) -> Vec<OsString> {
  let mut args = vec![OsString::from("ariadne")];

  for arg in string.split_whitespace() {
    args.push(arg.into());
  }

  args
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut maze: Arc<RwLock<Maze>> = Arc::new(RwLock::new(Maze::new(10, 10)));

  loop {
    print!("> ");
    let _ = stdout().flush()?;
    let mut cmd = String::new();
    stdin().read_line(&mut cmd).unwrap();
    cmd = cmd.replace("> ", "");

    match Cli::try_parse_from(string_to_args(&cmd)) {
      Ok(cli) => match cli.command {
        Commands::Quit | Commands::Exit => {
          std::process::exit(0);
        }
        Commands::Show => {
          println!("{}", maze.read().unwrap().string());
        }
        Commands::Load { file_path } => {
          let path = fs::canonicalize(file_path)?;
          maze = Arc::new(RwLock::new(Maze::from(path)));
        }
        Commands::Export { file_path } => {
          let path = fs::canonicalize(file_path)?;
          let mut file = fs::File::create(path)?;
          let s = serde_json::to_string_pretty(&maze.read().unwrap().clone())?;
          write!(file, "{}", s)?;
        }
        Commands::Clear => {
          std::process::Command::new("clear")
            .spawn()
            .unwrap()
            .wait()?;
        }
        Commands::Solve { visualize } => {
          let (tx, rx) = mpsc::channel::<HashSet<Coordinates>>();
          let visualize = visualize.unwrap_or(false);

          let send = Arc::clone(&maze);
          let handle = thread::spawn(move || {
            let mut solver = BFS::new(&send);
            return solver.solve(visualize, tx);
          });

          if visualize {
            execute!(
              stdout(),
              SetTitle("Ariadne"),
              DisableMouseCapture,
              Hide,
              EnterAlternateScreen,
              Clear(ClearType::Purge),
              MoveTo(0, 0),
            )?;

            for recv in &rx {
              let relevant = recv;
              let s = maze.read().unwrap().spread(relevant);
              println!("{}", s);
              execute!(stdout(), MoveTo(0, 0))?;
              thread::sleep(Duration::from_millis(50));
            }

            'solve_wait: loop {
              match read()? {
                _ => {
                  execute!(stdout(), LeaveAlternateScreen, Show)?;
                  break 'solve_wait;
                }
              }
            }
          } else {
            for _ in &rx {}
          }
          let solved = handle.join().unwrap();
          if solved {
            println!("{} Sucessfully solved maze", "✔".green().bold());
          } else {
            println!("{} Failed to sovle maze", "x".red().bold());
          }
        }
        Commands::Create {
          width,
          height,
          visualize,
        } => {
          let (tx, rx) = mpsc::channel::<Maze>();
          let visualize = visualize.unwrap_or(false);

          let handle = thread::spawn(move || {
            return Arc::new(RwLock::new(
              DFSOptions::new(width, height, visualize, tx).create(),
            ));
          });

          if visualize {
            execute!(
              stdout(),
              SetTitle("Ariadne"),
              DisableMouseCapture,
              Hide,
              EnterAlternateScreen,
              Clear(ClearType::Purge),
              MoveTo(0, 0),
            )?;

            for recv in &rx {
              println!("{}", recv.string());
              execute!(stdout(), MoveTo(0, 0))?;
              thread::sleep(Duration::from_millis(50));
            }

            'create_wait: loop {
              match read()? {
                _ => {
                  execute!(stdout(), LeaveAlternateScreen, Show)?;
                  break 'create_wait;
                }
              }
            }
          } else {
            for _ in &rx {}
          }
          maze = handle.join().unwrap();
          println!(
            "{} Created maze with size {}x{}",
            "✔".green().bold(),
            width,
            height
          );
        }
      },
      Err(e) => println!("{}", e),
    }
  }
}
