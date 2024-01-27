use std::error::Error;

use rustyline::Editor;

pub fn command(rl: &mut Editor<()>) -> Result<(), Box<dyn Error>> {
    rl.save_history(&ariadne::history())?;

    std::process::exit(0);
}
