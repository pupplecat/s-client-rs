mod program_test;
mod program_test_fixtures;
mod utils;

pub use program_test_fixtures::*;

pub type TestResult = Result<(), Box<dyn std::error::Error>>;
