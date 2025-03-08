use generic_pool_calculator_lib::{
    pda::CalculatorStateFindPdaArgs, utils::try_calculator_state_mut, GenericPoolSolValCalc,
    CALCULATOR_STATE_SIZE,
};
use marinade_calculator_lib::MarinadeSolValCalc;
use solana_program_test::{processor, ProgramTest};
use solana_sdk::{account::Account, pubkey::Pubkey};

use crate::utils::{ExtendedProgramTest, IntoAccount};

pub trait MarinadeCalculatorProgramTest {
    fn add_marinade_calculator_program(self) -> Self;

    fn add_marinade_prog(self) -> Self;

    fn add_marinade_stake_pool(self) -> Self;
}
pub const MARINADE_PROG_LAST_UPDATED_SLOT: u64 = 229_946_024;
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
        self.add_mock_calculator_state(MockCalculatorStateAccountArgs {
            manager: Pubkey::default(),
            last_upgrade_slot: MARINADE_PROG_LAST_UPDATED_SLOT,
            owner: MarinadeSolValCalc::ID,
        })
        .add_test_fixtures_account("marinade-prog.json")
        .add_test_fixtures_account("marinade-prog-data.json")
    }

    fn add_marinade_stake_pool(self) -> Self {
        self.add_test_fixtures_account("marinade-state.json")
            .add_test_fixtures_account("msol-mint.json")
    }
}

pub struct MockCalculatorStateAccountArgs {
    pub manager: Pubkey,
    pub last_upgrade_slot: u64,

    /// GenericPoolCalculator program ID
    pub owner: Pubkey,
}

impl IntoAccount for MockCalculatorStateAccountArgs {
    fn into_account(self) -> Account {
        let Self {
            manager,
            last_upgrade_slot,
            owner,
        } = self;
        let mut data = vec![0u8; CALCULATOR_STATE_SIZE];
        let state = try_calculator_state_mut(&mut data).unwrap();
        state.manager = manager;
        state.last_upgrade_slot = last_upgrade_slot;
        Account {
            lamports: 1_000_000_000, // just do 1 SOL lol
            data,
            owner,
            executable: false,
            rent_epoch: u64::MAX,
        }
    }
}

pub trait GenericPoolCalculatorProgramTest {
    fn add_mock_calculator_state(self, args: MockCalculatorStateAccountArgs) -> Self;
}

impl GenericPoolCalculatorProgramTest for ProgramTest {
    fn add_mock_calculator_state(self, args: MockCalculatorStateAccountArgs) -> Self {
        let (addr, _bump) = CalculatorStateFindPdaArgs {
            program_id: args.owner,
        }
        .get_calculator_state_address_and_bump_seed();
        self.add_account_chained(addr, args.into_account())
    }
}
