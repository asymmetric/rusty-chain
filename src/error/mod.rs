use std::error;
use std::fmt;

#[derive(Debug)]
pub enum MiningError {
    Iteration,
    NoParent,
}

impl fmt::Display for MiningError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MiningError::Iteration => write!(f, "could not mine block, hit iteration limit"),
            MiningError::NoParent => write!(f, "block has no parent"),
        }
    }
}

impl error::Error for MiningError {
    fn description(&self) -> &str {
        match *self {
            MiningError::Iteration => "could not mine block, hit iteration limit",
            MiningError::NoParent => "block has no parent",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
