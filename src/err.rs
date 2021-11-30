use const_format::formatcp;
use std::num::ParseIntError;

const USAGE: &'static str = "Usage: cargo run -- <Day> [Part]\n\t<Day> is a required Integer between 1 and 25\n\t[Part] is an optional Integer between 1 and 2. If not specified, both parts will run.";

pub struct ChristmasError {
    pub message: &'static str,
}

impl std::fmt::Display for ChristmasError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.message, USAGE)
    }
}

impl std::fmt::Debug for ChristmasError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.message, USAGE)
    }
}

impl From<ParseIntError> for ChristmasError {
    fn from(_: ParseIntError) -> Self {
        ChristmasError {
            message: formatcp!("Error during argument parsing.\n{}", USAGE),
        }
    }
}
