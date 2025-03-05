mod add_disable_pool_authority;
mod add_liquidity;
mod add_lst;
mod initialize;
mod set_admin;
mod set_pricing_program;
mod set_protocol_fee;
mod set_protocol_fee_beneficiary;
mod set_rebalance_authority;
mod set_sol_value_calculator;

pub use initialize::*;
pub use set_admin::*;
pub use set_pricing_program::*;
pub use set_protocol_fee_beneficiary::*;

use s_controller_interface::{LstState, PoolState};
use solana_program_test::BanksClientError;
use spl_token::state::Mint;
use std::sync::{Arc, Mutex};

use solana_sdk::{
    instruction::Instruction,
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signature},
    signer::Signer,
};

use crate::{utils::test_fixtures_dir, ProgramTestFixtures};

pub struct SProgramTestEnvironment {
    pub test_fixtures: Arc<Mutex<ProgramTestFixtures>>,
    pub authority: Keypair,
    pub rebalance_authority: Keypair,
    pub lp_mint: Pubkey,
}

impl SProgramTestEnvironment {
    pub async fn process_instructions(
        &self,
        instructions: &[Instruction],
        signers: &Vec<&Keypair>,
        payer: Option<&Keypair>,
    ) -> Result<Signature, BanksClientError> {
        let mut test_fixtures = self.test_fixtures.lock().unwrap();
        let signature = test_fixtures
            .program_simulator
            .process_ixs_with_default_compute_limit(instructions, signers, payer)
            .await?;

        Ok(signature)
    }

    pub async fn process_instruction(
        &self,
        instruction: Instruction,
        signers: &Vec<&Keypair>,
        payer: Option<&Keypair>,
    ) -> Result<Signature, BanksClientError> {
        self.process_instructions(&[instruction], signers, payer)
            .await
    }

    pub async fn get_pool_state(&self) -> Result<PoolState, Box<dyn std::error::Error>> {
        let mut test_fixtures = self.test_fixtures.lock().unwrap();
        let token_account = test_fixtures
            .program_simulator
            .get_borsh_account_data(self.get_pool_state_pubkey())
            .await?;

        Ok(token_account)
    }

    pub async fn get_mint_account(
        &self,
        mint_pubkey: Pubkey,
    ) -> Result<Mint, Box<dyn std::error::Error>> {
        let mut test_fixtures = self.test_fixtures.lock().unwrap();
        let mint = test_fixtures
            .program_simulator
            .get_packed_account_data(mint_pubkey)
            .await?;

        Ok(mint)
    }

    pub async fn get_mint_token_program(
        &self,
        mint_pubkey: Pubkey,
    ) -> Result<Pubkey, Box<dyn std::error::Error>> {
        let mut test_fixtures = self.test_fixtures.lock().unwrap();
        let mint = test_fixtures
            .program_simulator
            .get_account(mint_pubkey)
            .await?
            .unwrap();

        Ok(mint.owner)
    }

    // pub async fn get_lst_state_list(&self) -> Result<&[LstState], Box<dyn std::error::Error>> {
    //     let mut test_fixtures = self.test_fixtures.lock().unwrap();
    //     let token_account = test_fixtures
    //         .program_simulator
    //         .get_borsh_account_data(self.get_pool_state_pubkey())
    //         .await?;

    //     Ok(token_account)
    // }

    // pub async fn get_lst_state(&self) -> Result<LstState, Box<dyn std::error::Error>> {
    //     let mut test_fixtures = self.test_fixtures.lock().unwrap();
    //     let token_account = test_fixtures
    //         .program_simulator
    //         .get_borsh_account_data(self.get_lst_state_list_pubkey())
    //         .await?;

    //     Ok(token_account)
    // }

    pub fn get_program_id(&self) -> Pubkey {
        s_controller_lib::program::ID
    }

    pub fn get_pool_state_pubkey(&self) -> Pubkey {
        s_controller_lib::program::POOL_STATE_ID
    }

    pub fn get_lst_state_list_pubkey(&self) -> Pubkey {
        s_controller_lib::program::LST_STATE_LIST_ID
    }

    pub fn get_disable_pool_authority_list_pubkey(&self) -> Pubkey {
        s_controller_lib::program::DISABLE_POOL_AUTHORITY_LIST_ID
    }
}

pub async fn setup_s_program_test_environment() -> SProgramTestEnvironment {
    let mut test_fixtures = ProgramTestFixtures::setup_test_fixtures().await;

    let authority =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();

    test_fixtures
        .program_simulator
        .airdrop(&authority.pubkey(), 10 * LAMPORTS_PER_SOL)
        .await
        .unwrap();

    let lp_mint = test_fixtures
        .create_mint(&authority.pubkey(), 9)
        .await
        .unwrap();

    SProgramTestEnvironment {
        test_fixtures: Arc::new(Mutex::new(test_fixtures)),
        authority: authority.insecure_clone(),
        rebalance_authority: authority.insecure_clone(),
        lp_mint,
    }
}
