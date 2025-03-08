use marinade_calculator_lib::marinade_sol_val_calc_account_metas;
use s_controller_interface::{
    set_sol_value_calculator_ix, SetSolValueCalculatorIxArgs, SetSolValueCalculatorKeys,
};
use s_controller_lib::{
    find_pool_reserves_address, ix_extend_with_sol_value_calculator_accounts, FindLstPdaAtaKeys,
};
use solana_sdk::pubkey::Pubkey;

use crate::{utils::try_find_lst_state_index, TestResult};

use super::SProgramTestEnvironment;

impl SProgramTestEnvironment {
    pub async fn set_sol_value_calculator(&mut self, lst_mint_pubkey: Pubkey) -> TestResult {
        let pool_state = self.get_pool_state().await?;
        let lst_token_program = self.get_mint_token_program(lst_mint_pubkey).await?;

        let (pool_reserves_pubkey, _) = find_pool_reserves_address(FindLstPdaAtaKeys {
            lst_mint: lst_mint_pubkey,
            token_program: lst_token_program,
        });
        let lst_state_list = self.get_lst_state_list().await?;
        let (lst_index, _) =
            try_find_lst_state_index(&lst_state_list.unwrap_or_default(), lst_mint_pubkey)?;

        let mut set_sol_value_calculator_instruction = set_sol_value_calculator_ix(
            SetSolValueCalculatorKeys {
                admin: pool_state.admin,
                lst_mint: lst_mint_pubkey,
                pool_state: self.get_pool_state_pubkey(),
                pool_reserves: pool_reserves_pubkey,
                lst_state_list: self.get_lst_state_list_pubkey(),
            },
            SetSolValueCalculatorIxArgs {
                lst_index: lst_index as u32,
            },
        )?;

        ix_extend_with_sol_value_calculator_accounts(
            &mut set_sol_value_calculator_instruction,
            &marinade_sol_val_calc_account_metas(),
            marinade_calculator_lib::program::ID,
        )?;

        self.process_instruction(
            set_sol_value_calculator_instruction,
            &vec![&self.authority],
            Some(&self.payer),
        )
        .await?;

        Ok(())
    }
}
