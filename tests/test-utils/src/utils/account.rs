use solana_sdk::{account::Account, program_option::COption, program_pack::Pack, pubkey::Pubkey};
use spl_token::state::Mint;

use super::IntoAccount;

/// These might change in the future
pub const ZERO_SIZE_RENT_EXEMPT_LAMPORTS: u64 = 890_880;

/// These might change in the future
pub const RENT_EXEMPT_LAMPORT_PER_BYTE: u64 = 6960;

pub const fn est_rent_exempt_lamports(account_data_len: usize) -> u64 {
    ZERO_SIZE_RENT_EXEMPT_LAMPORTS + account_data_len as u64 * RENT_EXEMPT_LAMPORT_PER_BYTE
}

pub const TOKENKEG_ACC_RENT_EXEMPT_LAMPORTS: u64 =
    est_rent_exempt_lamports(spl_token::state::Account::LEN);

#[derive(Clone, Copy, Debug)]
pub struct MockTokenAccountArgs {
    pub mint: Pubkey,
    pub authority: Pubkey,
    pub amount: u64,
}

#[derive(Clone, Copy, Debug)]
pub struct MockMintArgs {
    pub mint_authority: Option<Pubkey>,
    pub freeze_authority: Option<Pubkey>,
    pub supply: u64,
    pub decimals: u8,
}

pub struct MockLpMintToInitArgs {
    pub initial_authority: Pubkey,
    pub addr: Pubkey,
}

pub fn mock_tokenkeg_account(
    MockTokenAccountArgs {
        mint,
        authority,
        amount,
    }: MockTokenAccountArgs,
) -> spl_token::state::Account {
    let is_native = mint == spl_token::native_mint::ID;
    spl_token::state::Account {
        mint,
        owner: authority,
        amount,
        delegate: COption::None,
        state: spl_token::state::AccountState::Initialized,
        is_native: if is_native {
            COption::Some(TOKENKEG_ACC_RENT_EXEMPT_LAMPORTS)
        } else {
            COption::None
        },
        delegated_amount: 0,
        close_authority: COption::None,
    }
}

impl IntoAccount for spl_token::state::Account {
    fn into_account(self) -> Account {
        let mut data = vec![0u8; spl_token::state::Account::LEN];
        let mut lamports = TOKENKEG_ACC_RENT_EXEMPT_LAMPORTS;
        if self.is_native.is_some() {
            lamports += self.amount;
        }
        spl_token::state::Account::pack(self, &mut data).unwrap();
        Account {
            lamports,
            data,
            owner: spl_token::ID,
            executable: false,
            rent_epoch: u64::MAX,
        }
    }
}

pub fn mock_tokenkeg_mint(
    MockMintArgs {
        mint_authority,
        freeze_authority,
        supply,
        decimals,
    }: MockMintArgs,
) -> Mint {
    Mint {
        mint_authority: COption::from(mint_authority),
        supply,
        decimals,
        is_initialized: true,
        freeze_authority: COption::from(freeze_authority),
    }
}

impl IntoAccount for Mint {
    fn into_account(self) -> Account {
        let mut data = vec![0u8; Mint::LEN];
        Mint::pack(self, &mut data).unwrap();
        Account {
            lamports: est_rent_exempt_lamports(Mint::LEN),
            data,
            owner: spl_token::ID,
            executable: false,
            rent_epoch: u64::MAX,
        }
    }
}
