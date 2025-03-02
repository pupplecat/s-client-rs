use s_controller_interface::{set_pricing_program_ix, SetPricingProgramKeys};
use solana_sdk::{pubkey::Pubkey, signer::Signer};

use crate::TestResult;

use super::SProgramTestEnvironment;

impl SProgramTestEnvironment {
    pub async fn set_pricing_program(&mut self, new_pricing_program_id: Pubkey) -> TestResult {
        let set_pricing_program_instruction = set_pricing_program_ix(SetPricingProgramKeys {
            new_pricing_program: new_pricing_program_id,
            pool_state: self.get_pool_state_pubkey(),
            admin: self.authority.pubkey(),
        })?;

        self.process_instruction(
            set_pricing_program_instruction,
            &vec![&self.authority],
            None,
        )
        .await?;

        Ok(())
    }
}
