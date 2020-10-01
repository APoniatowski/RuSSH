mod libs;  // Considering dropping this mod, as it is of no use

// use std::env;  // To be able to add arguments, further expansion is planned
use std::error::Error;
use std::env;
use crate::libs::run_types;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let run_type = &args[1]; // TODO expand on this later

    run_types::serial("config/pool.yml");
    Ok(())
}


// TODO write test function and create a test pool