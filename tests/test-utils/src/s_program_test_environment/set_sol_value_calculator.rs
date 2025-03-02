use s_controller_interface::{
    set_sol_value_calculator_ix, SetPricingProgramKeys, SetSolValueCalculatorIxArgs,
    SetSolValueCalculatorKeys,
};
use solana_sdk::{pubkey::Pubkey, signer::Signer};

use crate::TestResult;

use super::SProgramTestEnvironment;

impl SProgramTestEnvironment {
    pub async fn set_sol_value_calculator(&mut self, lst_index: u32) -> TestResult {
        let pool_state = self.get_pool_state().await?;

        let set_sol_value_calculator_instruction = set_sol_value_calculator_ix(
            SetSolValueCalculatorKeys {
                admin: self.authority.pubkey(),
                lst_mint,
                pool_state: self.get_pool_state_pubkey(),
                pool_reserves: todo!(),
                lst_state_list: todo!(),
            },
            SetSolValueCalculatorIxArgs { lst_index: todo!() },
        )?;

        self.process_instruction(
            set_sol_value_calculator_instruction,
            &vec![&self.authority],
            None,
        )
        .await?;

        Ok(())
    }
}
