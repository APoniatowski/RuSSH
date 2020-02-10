mod libs;  // Considering dropping this mod, as it is of no use

// use std::env;  // To be able to add arguments, further expansion is planned
use std::error::Error;
use crate::libs::ssh_lib;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    ssh_lib::serial("config/pool-testing.yml");
    Ok(())
}
