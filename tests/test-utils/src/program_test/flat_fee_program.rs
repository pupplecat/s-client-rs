use flat_fee_interface::ProgramState;
use flat_fee_lib::{
    initial_constants::{initial_manager, INITIAL_LP_WITHDRAWAL_FEE_BPS},
    program::STATE_SIZE,
    utils::try_program_state_mut,
};
use solana_program_test::{processor, ProgramTest};
use solana_sdk::account::Account;

use crate::utils::{est_rent_exempt_lamports, IntoAccount};

pub trait FlatFeeProgramTest {
    fn add_flat_fee_program(self) -> Self;

    fn add_no_fee_program(self) -> Self;
}

impl FlatFeeProgramTest for ProgramTest {
    fn add_flat_fee_program(mut self) -> Self {
        self.prefer_bpf(false);
        self.add_program(
            "flat_fee",
            flat_fee_lib::program::ID,
            processor!(flat_fee::entrypoint::process_instruction),
        );
        self
    }

    fn add_no_fee_program(mut self) -> Self {
        self.prefer_bpf(false);
        self.add_program(
            "no_fee_pricing_program",
            no_fee_pricing_program::ID,
            processor!(no_fee_pricing_program::process_instruction),
        );
        self
    }
}

pub const DEFAULT_PROGRAM_STATE: ProgramState = ProgramState {
    manager: initial_manager::ID,
    lp_withdrawal_fee_bps: INITIAL_LP_WITHDRAWAL_FEE_BPS,
};

pub struct MockProgramState(pub ProgramState);

impl IntoAccount for MockProgramState {
    fn into_account(self) -> Account {
        let mut data = vec![0u8; STATE_SIZE];
        let dst = try_program_state_mut(&mut data).unwrap();
        *dst = self.0;
        Account {
            lamports: est_rent_exempt_lamports(STATE_SIZE),
            data,
            owner: flat_fee_lib::program::ID,
            executable: false,
            rent_epoch: u64::MAX,
        }
    }
}
