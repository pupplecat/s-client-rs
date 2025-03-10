use s_controller_interface::{enable_lst_input_ix, EnableLstInputIxArgs, EnableLstInputKeys};
use solana_sdk::pubkey::Pubkey;

use crate::{utils::try_find_lst_state_index, TestResult};

use super::SProgramTestEnvironment;

impl SProgramTestEnvironment {
    pub async fn enable_lst_input(&mut self, lst_mint: Pubkey) -> TestResult {
        let pool_state = self.get_pool_state().await?;
        let lst_state_list = self.get_lst_state_list().await?.unwrap_or_default();
        let (lst_index, _) = try_find_lst_state_index(&lst_state_list, lst_mint)?;
        let enable_lst_input_instruction = enable_lst_input_ix(
            EnableLstInputKeys {
                admin: pool_state.admin,
                lst_mint,
                pool_state: self.get_pool_state_pubkey(),
                lst_state_list: self.get_lst_state_list_pubkey(),
            },
            EnableLstInputIxArgs {
                index: lst_index as u32,
            },
        )?;

        self.process_instruction(
            enable_lst_input_instruction,
            &vec![&self.authority],
            Some(&self.payer),
        )
        .await?;

        Ok(())
    }
}
