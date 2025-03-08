use s_controller_interface::{add_lst_ix, AddLstKeys};
use s_controller_lib::{
    find_pool_reserves_address, find_protocol_fee_accumulator_address, find_protocol_fee_address,
    FindLstPdaAtaKeys,
};
use solana_sdk::{pubkey::Pubkey, signer::Signer, system_program};

use crate::TestResult;

use super::SProgramTestEnvironment;

impl SProgramTestEnvironment {
    pub async fn add_lst(
        &mut self,
        lst_mint_pubkey: Pubkey,
        sol_value_calculator_pubkey: Pubkey,
    ) -> TestResult {
        let lst_token_program = self.get_mint_token_program(lst_mint_pubkey).await?;
        let pool_state = self.get_pool_state().await?;
        let (pool_reserves_pubkey, _) = find_pool_reserves_address(FindLstPdaAtaKeys {
            lst_mint: lst_mint_pubkey,
            token_program: lst_token_program,
        });
        let (protocol_fee_accumulator_pubkey, _) =
            find_protocol_fee_accumulator_address(FindLstPdaAtaKeys {
                lst_mint: lst_mint_pubkey,
                token_program: lst_token_program,
            });

        let (protocol_fee_accumulator_auth_pubkey, _) =
            find_protocol_fee_address(self.get_program_id());

        let add_lst_instruction = add_lst_ix(AddLstKeys {
            admin: pool_state.admin,
            payer: self.payer.pubkey(),
            lst_mint: lst_mint_pubkey,
            pool_reserves: pool_reserves_pubkey,
            protocol_fee_accumulator: protocol_fee_accumulator_pubkey,
            protocol_fee_accumulator_auth: protocol_fee_accumulator_auth_pubkey,
            sol_value_calculator: sol_value_calculator_pubkey,
            pool_state: self.get_pool_state_pubkey(),
            lst_state_list: self.get_lst_state_list_pubkey(),
            associated_token_program: spl_associated_token_account::ID,
            system_program: system_program::ID,
            lst_token_program,
        })?;

        self.process_instruction(
            add_lst_instruction,
            &vec![&self.authority],
            Some(&self.payer),
        )
        .await?;

        Ok(())
    }
}
