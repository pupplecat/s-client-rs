use s_controller_interface::{set_admin_ix, SetAdminKeys};
use solana_sdk::{signature::Keypair, signer::Signer};

use crate::TestResult;

use super::SProgramTestEnvironment;

impl SProgramTestEnvironment {
    pub async fn set_admin(&mut self, new_admin_keypair: &Keypair) -> TestResult {
        let set_admin_instruction = set_admin_ix(SetAdminKeys {
            current_admin: self.authority.pubkey(),
            new_admin: new_admin_keypair.pubkey(),
            pool_state: self.get_pool_state_pubkey(),
        })?;

        self.process_instruction(set_admin_instruction, &vec![&self.authority], None)
            .await?;

        self.authority = new_admin_keypair.insecure_clone();

        Ok(())
    }
}
