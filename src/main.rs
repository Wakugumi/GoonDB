use goondb::{Command, GoonEngine, GoonError};

fn run_repl() -> Result<(), GoonError> {
    let mut db = GoonEngine::open("my_gooning.log")?;

    Ok(())
}

fn main() {}
