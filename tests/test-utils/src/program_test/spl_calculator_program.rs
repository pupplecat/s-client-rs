use generic_pool_calculator_lib::GenericPoolSolValCalc;
use solana_program_test::{processor, ProgramTest};
use spl_calculator_lib::SplSolValCalc;

use crate::utils::ExtendedProgramTest;

use super::{GenericPoolCalculatorProgramTest, MockCalculatorStateAccountArgs};

pub trait SplCalculatorProgramTest {
    fn add_jito_stake_pool(self) -> Self;
    fn add_spl_progs(self) -> Self;
}

pub const SPL_STAKE_POOL_PROG_LAST_UPDATED_SLOT: u64 = 238_419_616;

impl SplCalculatorProgramTest for ProgramTest {
    fn add_spl_progs(mut self) -> Self {
        self.prefer_bpf(false);
        self.add_program(
            "spl_calculator",
            spl_calculator_lib::program::ID,
            processor!(spl_calculator::entrypoint::process_instruction),
        );
        self.add_mock_calculator_state(MockCalculatorStateAccountArgs {
            manager: spl_calculator_lib::initial_manager::ID,
            last_upgrade_slot: SPL_STAKE_POOL_PROG_LAST_UPDATED_SLOT,
            owner: SplSolValCalc::ID,
        })
        .add_test_fixtures_account("spl-stake-pool-prog.json")
        .add_test_fixtures_account("spl-stake-pool-prog-data.json")
    }

    fn add_jito_stake_pool(self) -> Self {
        self.add_test_fixtures_account("jito-stake-pool.json")
            .add_test_fixtures_account("jitosol-mint.json")
    }
}
