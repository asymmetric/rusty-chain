use std::error;
use std::fmt;

#[derive(Debug)]
pub struct MiningError;

impl fmt::Display for MiningError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not mine block, hit iteration limit")
    }
}

impl error::Error for MiningError {
    fn description(&self) -> &str {
        "could not mine bloc, hit iteration limit"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
