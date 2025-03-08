use s_controller_interface::PoolState;
use s_controller_lib::{
    initial_authority, program::POOL_STATE_ID, try_pool_state_mut, DEFAULT_PRICING_PROGRAM,
    POOL_STATE_SIZE,
};
use solana_program_test::ProgramTest;
use solana_sdk::{account::Account, pubkey::Pubkey};

use crate::utils::{est_rent_exempt_lamports, ExtendedProgramTest, IntoAccount};

pub const DEFAULT_POOL_STATE: PoolState = PoolState {
    total_sol_value: 0,
    trading_protocol_fee_bps: 0,
    lp_protocol_fee_bps: 0,
    version: 0,
    is_disabled: 0,
    is_rebalancing: 0,
    padding: [0u8; 1],
    admin: initial_authority::ID,
    rebalance_authority: initial_authority::ID,
    protocol_fee_beneficiary: initial_authority::ID,
    pricing_program: DEFAULT_PRICING_PROGRAM,
    lp_token_mint: Pubkey::new_from_array([0u8; 32]),
};

pub trait PoolStateProgramTest {
    fn add_pool_state(self, pool_state: PoolState) -> Self;
}

impl PoolStateProgramTest for ProgramTest {
    fn add_pool_state(self, pool_state: PoolState) -> Self {
        self.add_account_chained(POOL_STATE_ID, MockPoolState(pool_state).into_account())
    }
}

pub struct MockPoolState(pub PoolState);

impl IntoAccount for MockPoolState {
    fn into_account(self) -> Account {
        let mut data = vec![0u8; POOL_STATE_SIZE];
        let dst = try_pool_state_mut(&mut data).unwrap();
        *dst = self.0;
        Account {
            lamports: est_rent_exempt_lamports(POOL_STATE_SIZE),
            data,
            owner: s_controller_lib::program::ID,
            executable: false,
            rent_epoch: u64::MAX,
        }
    }
}
