use s_controller_lib::program::POOL_STATE_ID;
use solana_program_test::ProgramTest;
use solana_sdk::pubkey::Pubkey;

use crate::utils::{MockLpMintToInitArgs, MockMintArgs};

use super::token::TokenkegProgramTest;

pub trait LpTokenProgramTest {
    fn add_mock_lp_mint_to_init(self, args: MockLpMintToInitArgs) -> Self;
    fn add_mock_lp_mint(self, addr: Pubkey, supply: u64) -> Self;
}

impl LpTokenProgramTest for ProgramTest {
    fn add_mock_lp_mint_to_init(
        self,
        MockLpMintToInitArgs {
            initial_authority,
            addr,
        }: MockLpMintToInitArgs,
    ) -> Self {
        self.add_tokenkeg_mint_from_args(
            addr,
            MockMintArgs {
                mint_authority: Some(initial_authority),
                freeze_authority: Some(initial_authority),
                supply: 0,
                decimals: 9,
            },
        )
    }

    fn add_mock_lp_mint(self, addr: Pubkey, supply: u64) -> Self {
        self.add_tokenkeg_mint_from_args(
            addr,
            MockMintArgs {
                mint_authority: Some(POOL_STATE_ID),
                freeze_authority: Some(POOL_STATE_ID),
                supply,
                decimals: 9,
            },
        )
    }
}
