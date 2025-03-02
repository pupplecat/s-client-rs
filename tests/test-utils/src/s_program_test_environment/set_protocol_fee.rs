use s_controller_interface::{set_protocol_fee_ix, SetProtocolFeeIxArgs, SetProtocolFeeKeys};
use solana_sdk::signer::Signer;

use crate::TestResult;

use super::SProgramTestEnvironment;

impl SProgramTestEnvironment {
    pub async fn set_protocol_fee(
        &mut self,
        new_trading_protocol_fee_bps: Option<u16>,
        new_lp_protocol_fee_bps: Option<u16>,
    ) -> TestResult {
        let set_protocol_fee_instruction = set_protocol_fee_ix(
            SetProtocolFeeKeys {
                pool_state: self.get_pool_state_pubkey(),
                admin: self.authority.pubkey(),
            },
            SetProtocolFeeIxArgs {
                new_trading_protocol_fee_bps,
                new_lp_protocol_fee_bps,
            },
        )?;

        self.process_instruction(set_protocol_fee_instruction, &vec![&self.authority], None)
            .await?;

        Ok(())
    }
}
