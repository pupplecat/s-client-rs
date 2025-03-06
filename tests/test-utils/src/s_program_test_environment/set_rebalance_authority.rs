use s_controller_interface::{set_rebalance_authority_ix, SetRebalanceAuthorityKeys};
use solana_sdk::{signature::Keypair, signer::Signer};

use crate::TestResult;

use super::SProgramTestEnvironment;

impl SProgramTestEnvironment {
    pub async fn set_rebalance_authority(
        &mut self,
        new_rebalance_authority_keypair: &Keypair,
    ) -> TestResult {
        let set_rebalance_authority_instruction =
            set_rebalance_authority_ix(SetRebalanceAuthorityKeys {
                new_rebalance_authority: new_rebalance_authority_keypair.pubkey(),
                pool_state: self.get_pool_state_pubkey(),
                signer: self.authority.pubkey(), // can be admin or rebalance_authority
            })?;

        self.process_instruction(
            set_rebalance_authority_instruction,
            &vec![&self.authority],
            Some(&self.payer),
        )
        .await?;

        self.rebalance_authority = new_rebalance_authority_keypair.insecure_clone();

        Ok(())
    }
}
