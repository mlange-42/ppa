//! Command traits and implementations
use std::fmt;

pub trait Command {
    fn execute(&mut self) -> Result<(), CommandError> {
        Ok(())
    }
}

/// Error type for failed parsing of `String`s to `enum`s.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandError(String);

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
