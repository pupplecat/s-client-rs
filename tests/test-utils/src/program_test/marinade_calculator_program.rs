use solana_program_test::{processor, ProgramTest};

use crate::utils::ExtendedProgramTest;

pub trait MarinadeCalculatorProgramTest {
    fn add_marinade_calculator_program(self) -> Self;

    fn add_marinade_prog(self) -> Self;

    fn add_marinade_stake_pool(self) -> Self;
}

impl MarinadeCalculatorProgramTest for ProgramTest {
    fn add_marinade_calculator_program(mut self) -> Self {
        self.prefer_bpf(false);
        self.add_program(
            "marinade_calculator",
            marinade_calculator_lib::program::ID,
            processor!(marinade_calculator::entrypoint::process_instruction),
        );
        self
    }

    fn add_marinade_prog(self) -> Self {
        self.add_test_fixtures_account("marinade-prog.json")
            .add_test_fixtures_account("marinade-prog-data.json")
    }

    fn add_marinade_stake_pool(self) -> Self {
        self.add_test_fixtures_account("marinade-state.json")
            .add_test_fixtures_account("msol-mint.json")
    }
}
