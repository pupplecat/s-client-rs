use s_controller_interface::{
    remove_disable_pool_authority_ix, RemoveDisablePoolAuthorityIxArgs,
    RemoveDisablePoolAuthorityKeys,
};
use solana_sdk::{pubkey::Pubkey, signer::Signer};

use crate::{utils::try_find_disable_authority_index, TestResult};

use super::SProgramTestEnvironment;

impl SProgramTestEnvironment {
    pub async fn remove_disable_pool_authority(&mut self, authority: Pubkey) -> TestResult {
        let authority_list = self.get_disable_pool_authority_list().await?;

        let (index, _) =
            try_find_disable_authority_index(&authority_list.unwrap_or(vec![]), authority)?;

        let remove_disable_pool_authority_instruction = remove_disable_pool_authority_ix(
            RemoveDisablePoolAuthorityKeys {
                pool_state: self.get_pool_state_pubkey(),
                disable_pool_authority_list: self.get_disable_pool_authority_list_pubkey(),
                refund_rent_to: self.payer.pubkey(),
                signer: self.authority.pubkey(), // can be admin or the authority itself
                authority,
            },
            RemoveDisablePoolAuthorityIxArgs {
                index: index as u32,
            },
        )?;

        self.process_instruction(
            remove_disable_pool_authority_instruction,
            &vec![&self.authority],
            Some(&self.payer),
        )
        .await?;

        Ok(())
    }
}
