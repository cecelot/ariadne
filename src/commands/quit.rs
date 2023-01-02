use std::error::Error;

pub fn command() -> Result<(), Box<dyn Error>> {
    std::process::exit(0);
}
