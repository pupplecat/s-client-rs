use solana_program_test::ProgramTest;
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::pubkey::Pubkey;
use spl_token::state::Mint;

use crate::utils::{
    mock_tokenkeg_account, mock_tokenkeg_mint, ExtendedProgramTest, IntoAccount, MockMintArgs,
    MockTokenAccountArgs,
};

pub trait TokenkegProgramTest {
    fn add_tokenkeg_account(self, addr: Pubkey, account: spl_token::state::Account) -> Self;
    fn add_tokenkeg_account_from_args(self, addr: Pubkey, args: MockTokenAccountArgs) -> Self;
    fn add_tokenkeg_mint_account(self, addr: Pubkey, mint: Mint) -> Self;
    fn add_tokenkeg_mint_from_args(self, addr: Pubkey, args: MockMintArgs) -> Self;
}

impl<T: ExtendedProgramTest> TokenkegProgramTest for T {
    fn add_tokenkeg_account(self, addr: Pubkey, account: spl_token::state::Account) -> Self {
        self.add_keyed_account(KeyedAccount {
            pubkey: addr,
            account: account.into_account(),
        })
    }

    fn add_tokenkeg_account_from_args(self, addr: Pubkey, args: MockTokenAccountArgs) -> Self {
        self.add_tokenkeg_account(addr, mock_tokenkeg_account(args))
    }

    fn add_tokenkeg_mint_account(self, addr: Pubkey, mint: Mint) -> Self {
        self.add_keyed_account(KeyedAccount {
            pubkey: addr,
            account: mint.into_account(),
        })
    }

    fn add_tokenkeg_mint_from_args(self, addr: Pubkey, args: MockMintArgs) -> Self {
        self.add_tokenkeg_mint_account(addr, mock_tokenkeg_mint(args))
    }
}

pub trait GenAndAddTokenAccountProgramTest {
    fn gen_and_add_token_account(&mut self, args: MockTokenAccountArgs) -> Pubkey;
}

impl GenAndAddTokenAccountProgramTest for ProgramTest {
    fn gen_and_add_token_account(&mut self, args: MockTokenAccountArgs) -> Pubkey {
        let addr = Pubkey::new_unique();
        let token_acc = mock_tokenkeg_account(args);
        self.add_account(addr, token_acc.into_account());
        addr
    }
}
