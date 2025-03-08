use generic_pool_calculator_lib::GenericPoolSolValCalc;
use lido_calculator_lib::LidoSolValCalc;
use solana_program_test::{processor, ProgramTest};
use solana_sdk::pubkey::Pubkey;
use spl_calculator_lib::SplSolValCalc;

use crate::utils::ExtendedProgramTest;

use super::{GenericPoolCalculatorProgramTest, MockCalculatorStateAccountArgs};

pub trait LidoCalculatorProgramTest {
    fn add_lido_calculator_program(self) -> Self;

    fn add_lido_prog(self) -> Self;

    fn add_lido_stake_pool(self) -> Self;
}

pub const LIDO_PROG_LAST_UPDATED_SLOT: u64 = 165_468_732;

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
        let args = MockCalculatorStateAccountArgs {
            manager: Pubkey::default(),
            last_upgrade_slot: LIDO_PROG_LAST_UPDATED_SLOT,
            owner: LidoSolValCalc::ID,
        };
        self.add_mock_calculator_state(args)
            .add_test_fixtures_account("lido-prog.json")
            .add_test_fixtures_account("lido-prog-data.json")
    }

    fn add_lido_stake_pool(self) -> Self {
        self.add_test_fixtures_account("lido-state.json")
            .add_test_fixtures_account("stsol-mint.json")
    }
}
