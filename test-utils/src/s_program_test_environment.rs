use s_controller_interface::{initialize_ix, InitializeKeys};
use solana_program_test::BanksClientError;
use std::sync::{Arc, Mutex};

use solana_sdk::{
    instruction::Instruction,
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signature},
    signer::Signer,
    system_program,
};

use crate::{utils::resolve_path, ProgramTestFixtures};

pub struct SProgramTestEnvironment {
    pub test_fixtures: Arc<Mutex<ProgramTestFixtures>>,
    pub authority: Keypair,
    pub lp_mint: Pubkey,
}

impl SProgramTestEnvironment {
    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let initialize_instruction = initialize_ix(InitializeKeys {
            payer: self.authority.pubkey(),
            authority: self.authority.pubkey(),
            pool_state: self.get_pool_state_pubkey(),
            lp_token_mint: self.lp_mint,
            lp_token_program: spl_token::ID,
            system_program: system_program::ID,
        })?;

        self.process_instruction(initialize_instruction, &vec![&self.authority], None)
            .await?;

        Ok(())
    }

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

    pub fn get_program_id(&self) -> Pubkey {
        s_controller_lib::program::ID
    }

    pub fn get_pool_state_pubkey(&self) -> Pubkey {
        s_controller_lib::program::POOL_STATE_ID
    }
}

pub async fn setup_s_program_test_environment() -> SProgramTestEnvironment {
    let mut test_fixtures = ProgramTestFixtures::setup_test_fixtures().await;

    let authority = read_keypair_file(resolve_path(
        "test-fixtures/s-controller-test-authority-key.json",
    ))
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
        authority,
        lp_mint,
    }
}
