mod program_test;
mod program_test_fixtures;
mod s_program_test_environment;
mod utils;

pub use program_test_fixtures::*;
pub use s_program_test_environment::*;

pub type TestResult = Result<(), Box<dyn std::error::Error>>;
