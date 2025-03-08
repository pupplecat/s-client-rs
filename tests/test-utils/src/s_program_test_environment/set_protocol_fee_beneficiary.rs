use s_controller_interface::{set_protocol_fee_beneficiary_ix, SetProtocolFeeBeneficiaryKeys};
use solana_sdk::{signature::Keypair, signer::Signer};

use crate::TestResult;

use super::SProgramTestEnvironment;

impl SProgramTestEnvironment {
    pub async fn set_protocol_fee_beneficiary(
        &mut self,
        new_protocol_fee_beneficiary: &Keypair,
    ) -> TestResult {
        let pool_state = self.get_pool_state().await?;

        let set_protocol_fee_beneficiary_instruction =
            set_protocol_fee_beneficiary_ix(SetProtocolFeeBeneficiaryKeys {
                current_beneficiary: pool_state.protocol_fee_beneficiary,
                new_beneficiary: new_protocol_fee_beneficiary.pubkey(),
                pool_state: self.get_pool_state_pubkey(),
            })?;

        self.process_instruction(
            set_protocol_fee_beneficiary_instruction,
            &vec![&self.protocol_fee_beneficiary],
            Some(&self.payer),
        )
        .await?;

        self.protocol_fee_beneficiary = new_protocol_fee_beneficiary.insecure_clone();

        Ok(())
    }
}
