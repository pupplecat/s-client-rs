use solana_program_test::{processor, ProgramTest};

pub trait MarinadeCalculatorProgramTest {
    fn add_marinade_calculator_program(self) -> Self;
}

impl MarinadeCalculatorProgramTest for ProgramTest {
    fn add_marinade_calculator_program(mut self) -> Self {
        self.add_program(
            "marinade_calculator",
            marinade_calculator_lib::program::ID,
            processor!(marinade_calculator::entrypoint::process_instruction),
        );
        self
    }
}
