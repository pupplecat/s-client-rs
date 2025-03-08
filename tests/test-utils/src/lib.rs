pub mod program_test;
pub mod program_test_fixtures;
pub mod s_program_test_environment;
pub mod utils;

pub use program_test_fixtures::*;
pub use s_program_test_environment::*;

pub type TestResult = Result<(), Box<dyn std::error::Error>>;
