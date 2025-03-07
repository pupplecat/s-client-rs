use solana_program_test::{processor, ProgramTest};

use crate::utils::ExtendedProgramTest;

pub trait LidoCalculatorProgramTest {
    fn add_lido_calculator_program(self) -> Self;

    fn add_lido_prog(self) -> Self;

    fn add_lido_stake_pool(self) -> Self;
}

impl LidoCalculatorProgramTest for ProgramTest {
    fn add_lido_calculator_program(mut self) -> Self {
        self.prefer_bpf(false);
        self.add_program(
            "lido_calculator",
            lido_calculator_lib::program::ID,
            processor!(lido_calculator::entrypoint::process_instruction),
        );
        self
    }

    fn add_lido_prog(self) -> Self {
        self.add_test_fixtures_account("lido-prog.json")
            .add_test_fixtures_account("lido-prog-data.json")
    }

    fn add_lido_stake_pool(self) -> Self {
        self.add_test_fixtures_account("lido-state.json")
            .add_test_fixtures_account("stsol-mint.json")
    }
}
