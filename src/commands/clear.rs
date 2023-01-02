use std::error::Error;

pub fn command() -> Result<(), Box<dyn Error>> {
    std::process::Command::new("clear")
        .spawn()
        .unwrap()
        .wait()?;

    Ok(())
}
