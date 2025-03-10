use marinade_calculator_lib::marinade_sol_val_calc_account_metas;
use marinade_keys::msol;
use s_controller_interface::AddLiquidityKeys;
use s_controller_lib::{
    add_liquidity_ix_full, find_pool_reserves_address, find_protocol_fee_accumulator_address,
    AddLiquidityIxAmts, AddLiquidityIxFullArgs, AddRemoveLiquidityExtraAccounts, FindLstPdaAtaKeys,
};
use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey, signature::Keypair, signer::Signer};
use spl_associated_token_account::get_associated_token_address_with_program_id;

use crate::{
    utils::{jitosol, try_find_lst_state_index},
    TestResult,
};

use super::SProgramTestEnvironment;

impl SProgramTestEnvironment {
    // pub async fn add_liquidity(
    //     &mut self,
    //     lst_mint_pubkey: Pubkey,
    //     lst_amount: u64,
    //     signer: Keypair,
    // ) -> TestResult {
    //     let pool_state = self.get_pool_state().await?;
    //     let lst_token_program = self.get_mint_token_program(lst_mint_pubkey).await?;
    //     let lp_token_program = self
    //         .get_mint_token_program(pool_state.lp_token_mint)
    //         .await?;

    //     let src_lst_acc = get_associated_token_address_with_program_id(
    //         &signer.pubkey(),
    //         &lst_mint_pubkey,
    //         &lst_token_program,
    //     );

    //     let dst_lp_acc = get_associated_token_address_with_program_id(
    //         &signer.pubkey(),
    //         &pool_state.lp_token_mint,
    //         &lp_token_program,
    //     );

    //     let (protocol_fee_accumulator_pubkey, _) =
    //         find_protocol_fee_accumulator_address(FindLstPdaAtaKeys {
    //             lst_mint: lst_mint_pubkey,
    //             token_program: lst_token_program,
    //         });

    //     let (pool_reserves_pubkey, _) = find_pool_reserves_address(FindLstPdaAtaKeys {
    //         lst_mint: lst_mint_pubkey,
    //         token_program: lst_token_program,
    //     });
    //     let lst_state_list = self.get_lst_state_list().await?.unwrap_or_default();
    //     let (lst_index, _) = try_find_lst_state_index(&lst_state_list, lst_mint_pubkey)?;

    //     let add_liquidity_instruction = add_liquidity_ix(
    //         AddLiquidityKeys {
    //             signer: signer.pubkey(),
    //             lst_mint: lst_mint_pubkey,
    //             src_lst_acc,
    //             dst_lp_acc,
    //             lp_token_mint: pool_state.lp_token_mint,
    //             protocol_fee_accumulator: protocol_fee_accumulator_pubkey,
    //             lst_token_program: lst_token_program,
    //             lp_token_program: lp_token_program,
    //             pool_state: self.get_pool_state_pubkey(),
    //             lst_state_list: self.get_lst_state_list_pubkey(),
    //             pool_reserves: pool_reserves_pubkey,
    //         },
    //         AddLiquidityIxArgs {
    //             lst_value_calc_accs: 0, // TODO: AddLiquidityIxArgs.lst_value_calc_accs
    //             lst_index: lst_index as u32,
    //             lst_amount: lst_amount,
    //             min_lp_out: 0,
    //         },
    //     )?;

    //     self.process_instruction(add_liquidity_instruction, &vec![&signer], None)
    //         .await?;

    //     Ok(())
    // }

    pub async fn add_liquidity(
        &mut self,
        lst_mint_pubkey: Pubkey,
        lst_amount: u64,
        liquidity_provider: Keypair,
    ) -> TestResult {
        println!("xxx 1");
        let pool_state = self.get_pool_state().await?;
        let lst_token_program = self.get_mint_token_program(lst_mint_pubkey).await?;
        let lp_token_program = self
            .get_mint_token_program(pool_state.lp_token_mint)
            .await?;
        println!("xxx 2");
        let src_lst_acc = get_associated_token_address_with_program_id(
            &liquidity_provider.pubkey(),
            &lst_mint_pubkey,
            &lst_token_program,
        );
        println!("xxx src_lst_acc {}", src_lst_acc);
        println!("xxx 3");
        let dst_lp_acc = get_associated_token_address_with_program_id(
            &liquidity_provider.pubkey(),
            &pool_state.lp_token_mint,
            &lp_token_program,
        );
        println!("xxx dst_lp_acc {}", dst_lp_acc);
        println!("xxx 4");
        let (protocol_fee_accumulator_pubkey, _) =
            find_protocol_fee_accumulator_address(FindLstPdaAtaKeys {
                lst_mint: lst_mint_pubkey,
                token_program: lst_token_program,
            });
        println!("xxx 4");
        let (pool_reserves_pubkey, _) = find_pool_reserves_address(FindLstPdaAtaKeys {
            lst_mint: lst_mint_pubkey,
            token_program: lst_token_program,
        });
        let lst_state_list = self.get_lst_state_list().await?.unwrap_or_default();
        let (lst_index, _) = try_find_lst_state_index(&lst_state_list, lst_mint_pubkey)?;
        println!("xxx 5");

        println!("xxx add_liquidity_keys");
        let add_liquidity_keys = AddLiquidityKeys {
            signer: liquidity_provider.pubkey(),
            lst_mint: lst_mint_pubkey,
            src_lst_acc, //lst_account_to_add_from
            dst_lp_acc,  //liquidity_provider_lp_token_acc_addr
            lp_token_mint: pool_state.lp_token_mint,
            protocol_fee_accumulator: protocol_fee_accumulator_pubkey,
            lst_token_program: lst_token_program,
            lp_token_program: lp_token_program,
            pool_state: self.get_pool_state_pubkey(),
            lst_state_list: self.get_lst_state_list_pubkey(),
            pool_reserves: pool_reserves_pubkey,
        };
        println!("xxx add_liquidity_instruction");
        let add_liquidity_instruction = add_liquidity_ix_full(
            add_liquidity_keys,
            AddLiquidityIxFullArgs {
                lst_index,
                amts: AddLiquidityIxAmts {
                    lst_amount: lst_amount,
                    min_lp_out: 0,
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
        println!("xxx process_instruction");
        self.process_instruction(add_liquidity_instruction, &vec![&liquidity_provider], None)
            .await?;

        Ok(())
    }
}
