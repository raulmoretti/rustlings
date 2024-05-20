// modules3.rs
//
// You can use the 'use' keyword to bring module paths from modules from
// anywhere and especially from the Rust standard library into your scope. Bring
// SystemTime and UNIX_EPOCH from the std::time module. Bonus style points if
// you can do it with one line!
//
// Execute `rustlings hint modules3` or use the `hint` watch subcommand for a
// hint.



// TODO: Complete this use statement
use std::time::{SystemTime, UNIX_EPOCH, SystemTimeError};

fn main() {
    match seconds_to_years() {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} years ago!", n),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

fn seconds_to_years() -> Result<f64, SystemTimeError> {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH)?;
    Ok(duration.as_secs() as f64 / (365.2422 * 24.0 * 60.0 * 60.0))
}
