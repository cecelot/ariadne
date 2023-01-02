use std::error::Error;

use rustyline::Editor;

pub fn command(home: &str, rl: &mut Editor<()>) -> Result<(), Box<dyn Error>> {
    let path = format!("{}/.ariadne_history", home);
    rl.save_history(&path)?;

    std::process::exit(0);
}
