use s_controller_interface::{add_disable_pool_authority_ix, AddDisablePoolAuthorityKeys};
use solana_sdk::{pubkey::Pubkey, system_program};

use crate::TestResult;

use super::SProgramTestEnvironment;

impl SProgramTestEnvironment {
    pub async fn add_disable_pool_authority(&mut self, new_authority: Pubkey) -> TestResult {
        let pool_state = self.get_pool_state().await?;

        let add_disable_pool_authority_instruction =
            add_disable_pool_authority_ix(AddDisablePoolAuthorityKeys {
                payer: pool_state.admin, // signer
                admin: pool_state.admin, // signer
                pool_state: self.get_pool_state_pubkey(),
                new_authority,
                disable_pool_authority_list: self.get_disable_pool_authority_list_pubkey(),
                system_program: system_program::ID,
            })?;

        self.process_instruction(
            add_disable_pool_authority_instruction,
            &vec![&self.authority],
            None,
        )
        .await?;

        Ok(())
    }
}
