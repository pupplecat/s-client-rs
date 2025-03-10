use s_controller_interface::{disable_lst_input_ix, DisableLstInputIxArgs, DisableLstInputKeys};
use solana_sdk::pubkey::Pubkey;

use crate::{utils::try_find_lst_state_index, TestResult};

use super::SProgramTestEnvironment;

impl SProgramTestEnvironment {
    pub async fn disable_lst_input(&mut self, lst_mint: Pubkey) -> TestResult {
        let pool_state = self.get_pool_state().await?;
        let lst_state_list = self.get_lst_state_list().await?.unwrap_or_default();
        let (lst_index, _) = try_find_lst_state_index(&lst_state_list, lst_mint)?;
        let disable_lst_input_instruction = disable_lst_input_ix(
            DisableLstInputKeys {
                admin: pool_state.admin,
                lst_mint,
                pool_state: self.get_pool_state_pubkey(),
                lst_state_list: self.get_lst_state_list_pubkey(),
            },
            DisableLstInputIxArgs {
                index: lst_index as u32,
            },
        )?;

        self.process_instruction(
            disable_lst_input_instruction,
            &vec![&self.authority],
            Some(&self.payer),
        )
        .await?;

        Ok(())
    }
}
