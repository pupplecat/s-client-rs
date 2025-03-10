use s_controller_interface::{disable_pool_ix, DisablePoolKeys};

use crate::TestResult;

use super::SProgramTestEnvironment;

impl SProgramTestEnvironment {
    pub async fn disable_pool(&mut self) -> TestResult {
        let pool_state = self.get_pool_state().await?;

        let disable_pool_instruction = disable_pool_ix(DisablePoolKeys {
            signer: pool_state.admin, // admin or one of disable pool authority
            pool_state: self.get_pool_state_pubkey(),
            disable_pool_authority_list: self.get_disable_pool_authority_list_pubkey(),
        })?;

        self.process_instruction(
            disable_pool_instruction,
            &vec![&self.authority],
            Some(&self.payer),
        )
        .await?;

        Ok(())
    }
}
