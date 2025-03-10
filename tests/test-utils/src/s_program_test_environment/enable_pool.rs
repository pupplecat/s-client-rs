use s_controller_interface::{enable_pool_ix, EnablePoolKeys};

use crate::TestResult;

use super::SProgramTestEnvironment;

impl SProgramTestEnvironment {
    pub async fn enable_pool(&mut self) -> TestResult {
        let pool_state = self.get_pool_state().await?;

        let enable_pool_instruction = enable_pool_ix(EnablePoolKeys {
            admin: pool_state.admin,
            pool_state: self.get_pool_state_pubkey(),
        })?;

        self.process_instruction(
            enable_pool_instruction,
            &vec![&self.authority],
            Some(&self.payer),
        )
        .await?;

        Ok(())
    }
}
