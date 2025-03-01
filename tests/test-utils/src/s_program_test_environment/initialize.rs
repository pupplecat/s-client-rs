use s_controller_interface::{initialize_ix, InitializeKeys};
use solana_sdk::{signer::Signer, system_program};

use crate::TestResult;

use super::SProgramTestEnvironment;

pub trait SProgramInitialize {
    async fn initialize(&mut self) -> TestResult;
}

impl SProgramInitialize for SProgramTestEnvironment {
    async fn initialize(&mut self) -> TestResult {
        let authority = self.authority.pubkey();
        let initialize_instruction = initialize_ix(InitializeKeys {
            payer: self.authority.pubkey(),
            authority,
            pool_state: self.get_pool_state_pubkey(),
            lp_token_mint: self.lp_mint,
            lp_token_program: spl_token::ID,
            system_program: system_program::ID,
        })?;

        self.process_instruction(initialize_instruction, &vec![&self.authority], None)
            .await?;

        Ok(())
    }
}
