mod add_disable_pool_authority;
mod add_liquidity;
mod add_lst;
mod initialize;
mod remove_disable_pool_authority;
mod remove_lst;
mod set_admin;
mod set_pricing_program;
mod set_protocol_fee;
mod set_protocol_fee_beneficiary;
mod set_rebalance_authority;
mod set_sol_value_calculator;

use bytemuck::AnyBitPattern;

use s_controller_interface::{LstState, PoolState};
use solana_program_test::{BanksClientError, ProgramTest};
use spl_token::state::{Account, Mint};
use std::sync::{Arc, Mutex};

use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signature},
    signer::Signer,
};

use crate::{utils::test_fixtures_dir, ProgramTestFixtures};

pub struct SProgramTestEnvironment {
    pub test_fixtures: Arc<Mutex<ProgramTestFixtures>>,
    pub payer: Keypair,
    pub authority: Keypair,
    pub rebalance_authority: Keypair,
    pub protocol_fee_beneficiary: Keypair,
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

    pub async fn get_token_account_by_pubkey(
        &self,
        account: &Pubkey,
    ) -> Result<Account, Box<dyn std::error::Error>> {
        let mut test_fixtures = self.test_fixtures.lock().unwrap();
        let token_account = test_fixtures
            .program_simulator
            .get_packed_account_data(*account)
            .await?;

        Ok(token_account)
    }

    pub async fn get_lst_state_list(
        &self,
    ) -> Result<Option<Vec<LstState>>, Box<dyn std::error::Error>> {
        let mut test_fixtures = self.test_fixtures.lock().unwrap();
        let account = test_fixtures
            .program_simulator
            .get_account(self.get_lst_state_list_pubkey())
            .await?;

        if account.is_none() {
            return Ok(None);
        }

        let account = account.unwrap();
        // Get the slice from the account data.
        let list_slice = try_list(&account.data);
        // Convert it to a Vec if it exists.
        Ok(list_slice.map(|s| s.to_vec()))
    }

    pub async fn get_disable_pool_authority_list(
        &self,
    ) -> Result<Option<Vec<Pubkey>>, Box<dyn std::error::Error>> {
        let mut test_fixtures = self.test_fixtures.lock().unwrap();
        let account = test_fixtures
            .program_simulator
            .get_account(self.get_disable_pool_authority_list_pubkey())
            .await?;

        if account.is_none() {
            return Ok(None);
        }

        let account = account.unwrap();

        // Get the slice from the account data.
        let list_slice = try_list(&account.data);
        // Convert it to a Vec if it exists.
        Ok(list_slice.map(|s| s.to_vec()))
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

    pub fn get_protocol_fee_auth_pubkey(&self) -> Pubkey {
        s_controller_lib::program::PROTOCOL_FEE_ID
    }
}

pub async fn setup_s_program_test_environment() -> SProgramTestEnvironment {
    let mut test_fixtures: ProgramTestFixtures = ProgramTestFixtures::setup_test_fixtures().await;

    let payer = test_fixtures
        .program_simulator
        .get_funded_keypair()
        .await
        .unwrap();

    let authority =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();

    let lp_mint = test_fixtures
        .create_mint(&authority.pubkey(), 9)
        .await
        .unwrap();

    SProgramTestEnvironment {
        test_fixtures: Arc::new(Mutex::new(test_fixtures)),
        payer,
        authority: authority.insecure_clone(),
        rebalance_authority: authority.insecure_clone(),
        protocol_fee_beneficiary: authority.insecure_clone(),
        lp_mint,
    }
}

pub async fn setup_s_program_test_environment_with_program_test(
    program_test: ProgramTest,
) -> SProgramTestEnvironment {
    let mut test_fixtures: ProgramTestFixtures = ProgramTestFixtures::setup(program_test).await;

    let payer = test_fixtures
        .program_simulator
        .get_funded_keypair()
        .await
        .unwrap();

    let authority =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();

    let lp_mint = test_fixtures
        .create_mint(&authority.pubkey(), 9)
        .await
        .unwrap();

    SProgramTestEnvironment {
        test_fixtures: Arc::new(Mutex::new(test_fixtures)),
        payer,
        authority: authority.insecure_clone(),
        rebalance_authority: authority.insecure_clone(),
        protocol_fee_beneficiary: authority.insecure_clone(),
        lp_mint,
    }
}

fn try_list<T: AnyBitPattern>(list_acc_data: &[u8]) -> Option<&[T]> {
    if list_acc_data.len() % std::mem::size_of::<T>() != 0 {
        return None;
    }
    let ptr = list_acc_data.as_ptr();
    if ptr.align_offset(std::mem::align_of::<T>()) != 0 {
        return None;
    }
    let len = list_acc_data.len() / std::mem::size_of::<T>();
    Some(unsafe { std::slice::from_raw_parts(ptr as *const T, len) })
}
