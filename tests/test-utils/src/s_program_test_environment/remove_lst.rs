use s_controller_interface::{remove_lst_ix, RemoveLstIxArgs, RemoveLstKeys};
use s_controller_lib::{
    find_pool_reserves_address, find_protocol_fee_accumulator_address, find_protocol_fee_address,
    FindLstPdaAtaKeys,
};
use solana_sdk::{pubkey::Pubkey, signer::Signer};

use crate::{utils::try_find_lst_state_index, TestResult};

use super::SProgramTestEnvironment;

impl SProgramTestEnvironment {
    pub async fn remove_lst(&mut self, lst_mint_pubkey: Pubkey) -> TestResult {
        let lst_token_program = self.get_mint_token_program(lst_mint_pubkey).await?;
        let lst_state_list = self.get_lst_state_list().await?;
        let (lst_index, _) =
            try_find_lst_state_index(&lst_state_list.unwrap_or_default(), lst_mint_pubkey)?;

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

        let remove_lst_instruction = remove_lst_ix(
            RemoveLstKeys {
                lst_mint: lst_mint_pubkey,
                admin: pool_state.admin,
                pool_reserves: pool_reserves_pubkey,
                protocol_fee_accumulator: protocol_fee_accumulator_pubkey,
                protocol_fee_accumulator_auth: protocol_fee_accumulator_auth_pubkey,
                pool_state: self.get_pool_state_pubkey(),
                lst_state_list: self.get_lst_state_list_pubkey(),
                lst_token_program,
                refund_rent_to: self.payer.pubkey(),
            },
            RemoveLstIxArgs {
                lst_index: lst_index as u32,
            },
        )?;

        self.process_instruction(
            remove_lst_instruction,
            &vec![&self.authority],
            Some(&self.payer),
        )
        .await?;

        Ok(())
    }
}
