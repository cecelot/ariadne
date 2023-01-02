use crate::{lib::prelude::*, Coordinates};
use std::{
    collections::HashSet,
    error::Error,
    io::stdout,
    sync::{mpsc, Arc, RwLock},
    thread,
    time::Duration,
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, DisableMouseCapture},
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetTitle},
};

use colored::Colorize;

pub fn command(maze: Arc<RwLock<Maze>>, visualize: Option<bool>) -> Result<(), Box<dyn Error>> {
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

    Ok(())
}
