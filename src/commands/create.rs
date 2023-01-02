use crate::lib::prelude::*;
use std::{
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

pub fn command(
    visualize: Option<bool>,
    width: usize,
    height: usize,
) -> Result<Arc<RwLock<Maze>>, Box<dyn Error>> {
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
    
    let maze = handle.join().unwrap();
    
    println!(
        "{} Created maze with size {}x{}",
        "✔".green().bold(),
        width,
        height
    );

    Ok(maze)
}
