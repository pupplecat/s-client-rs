use s_controller_interface::{add_liquidity_ix, AddLiquidityIxArgs, AddLiquidityKeys};
use s_controller_lib::{
    find_pool_reserves_address, find_protocol_fee_accumulator_address, FindLstPdaAtaKeys,
};
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};
use spl_associated_token_account::get_associated_token_address_with_program_id;

use crate::TestResult;

use super::SProgramTestEnvironment;

impl SProgramTestEnvironment {
    pub async fn add_liquidity(
        &mut self,
        lst_mint_pubkey: Pubkey,
        lst_amount: u64,
        signer: Keypair,
    ) -> TestResult {
        let pool_state = self.get_pool_state().await?;
        let lst_token_program = self.get_mint_token_program(lst_mint_pubkey).await?;
        let lp_token_program = self
            .get_mint_token_program(pool_state.lp_token_mint)
            .await?;

        let src_lst_acc = get_associated_token_address_with_program_id(
            &signer.pubkey(),
            &lst_mint_pubkey,
            &lst_token_program,
        );

        let dst_lp_acc = get_associated_token_address_with_program_id(
            &signer.pubkey(),
            &pool_state.lp_token_mint,
            &lp_token_program,
        );

        let (protocol_fee_accumulator_pubkey, _) =
            find_protocol_fee_accumulator_address(FindLstPdaAtaKeys {
                lst_mint: lst_mint_pubkey,
                token_program: lst_token_program,
            });

        let (pool_reserves_pubkey, _) = find_pool_reserves_address(FindLstPdaAtaKeys {
            lst_mint: lst_mint_pubkey,
            token_program: lst_token_program,
        });

        let add_liquidity_instruction = add_liquidity_ix(
            AddLiquidityKeys {
                signer: signer.pubkey(),
                lst_mint: lst_mint_pubkey,
                src_lst_acc,
                dst_lp_acc,
                lp_token_mint: pool_state.lp_token_mint,
                protocol_fee_accumulator: protocol_fee_accumulator_pubkey,
                lst_token_program: lst_token_program,
                lp_token_program: lp_token_program,
                pool_state: self.get_pool_state_pubkey(),
                lst_state_list: self.get_lst_state_list_pubkey(),
                pool_reserves: pool_reserves_pubkey,
            },
            AddLiquidityIxArgs {
                lst_value_calc_accs: 0, // TODO: AddLiquidityIxArgs.lst_value_calc_accs
                lst_index: 0,
                lst_amount: lst_amount,
                min_lp_out: 0,
            },
        )?;

        self.process_instruction(add_liquidity_instruction, &vec![&signer], None)
            .await?;

        Ok(())
    }
}
