use marinade_calculator_lib::marinade_sol_val_calc_account_metas;
use marinade_keys::msol;
use s_controller_interface::RemoveLiquidityKeys;
use s_controller_lib::{
    find_pool_reserves_address, find_protocol_fee_accumulator_address, remove_liquidity_ix_full,
    AddRemoveLiquidityExtraAccounts, FindLstPdaAtaKeys, RemoveLiquidityIxAmts,
    RemoveLiquidityIxFullArgs,
};
use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey, signature::Keypair, signer::Signer};
use spl_associated_token_account::get_associated_token_address_with_program_id;

use crate::{utils::try_find_lst_state_index, TestResult};

use super::SProgramTestEnvironment;

impl SProgramTestEnvironment {
    pub async fn remove_liquidity(
        &mut self,
        lst_mint_pubkey: Pubkey,
        lp_token_amount: u64,
        liquidity_provider: Keypair,
    ) -> TestResult {
        let pool_state = self.get_pool_state().await?;
        let lst_token_program = self.get_mint_token_program(lst_mint_pubkey).await?;
        let lp_token_program = self
            .get_mint_token_program(pool_state.lp_token_mint)
            .await?;

        let dst_lst_acc = get_associated_token_address_with_program_id(
            &liquidity_provider.pubkey(),
            &lst_mint_pubkey,
            &lst_token_program,
        );

        let src_lp_acc = get_associated_token_address_with_program_id(
            &liquidity_provider.pubkey(),
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
        let lst_state_list = self.get_lst_state_list().await?.unwrap_or_default();
        let (lst_index, _) = try_find_lst_state_index(&lst_state_list, lst_mint_pubkey)?;

        let remove_liquidity_keys = RemoveLiquidityKeys {
            signer: liquidity_provider.pubkey(),
            lst_mint: lst_mint_pubkey,
            dst_lst_acc,
            src_lp_acc,
            lp_token_mint: pool_state.lp_token_mint,
            protocol_fee_accumulator: protocol_fee_accumulator_pubkey,
            lst_token_program: lst_token_program,
            lp_token_program: lp_token_program,
            pool_state: self.get_pool_state_pubkey(),
            lst_state_list: self.get_lst_state_list_pubkey(),
            pool_reserves: pool_reserves_pubkey,
        };

        let remove_liquidity_instruction = remove_liquidity_ix_full(
            remove_liquidity_keys,
            RemoveLiquidityIxFullArgs {
                lst_index,
                amts: RemoveLiquidityIxAmts {
                    lp_token_amount,
                    min_lst_out: 0,
                },
            },
            AddRemoveLiquidityExtraAccounts {
                lst_calculator_program_id: marinade_calculator_lib::program::ID,
                pricing_program_id: no_fee_pricing_program::ID,
                lst_calculator_accounts: &marinade_sol_val_calc_account_metas(),
                pricing_program_price_lp_accounts: &[AccountMeta {
                    pubkey: msol::ID,
                    is_signer: false,
                    is_writable: false,
                }],
            },
        )?;

        self.process_instruction(
            remove_liquidity_instruction,
            &vec![&liquidity_provider],
            None,
        )
        .await?;

        Ok(())
    }
}
