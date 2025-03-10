use marinade_calculator_lib::marinade_sol_val_calc_account_metas;
use marinade_keys::msol;
use s_controller_lib::{
    end_rebalance_ix_full,
    program::{LST_STATE_LIST_ID, POOL_STATE_ID},
    start_rebalance_ix_full, EndRebalanceFromStartRebalanceKeys, SrcDstLstIndexes,
    SrcDstLstSolValueCalcAccounts, StartRebalanceByMintsFreeArgs, StartRebalanceIxFullArgs,
    StartRebalanceIxLstAmts,
};
use solana_program::instruction::Instruction;
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};
use spl_calculator_lib::SplLstSolCommonFreeArgsConst;

use crate::{
    utils::{
        jito_stake_pool, transfer_checked_ix, MintWithTokenProgram, TransferCheckedArgs,
        TransferCheckedKeys,
    },
    TestResult,
};

use super::SProgramTestEnvironment;

impl SProgramTestEnvironment {
    pub async fn rebalance_ixs(
        &mut self,
        withdraw_to_pubkey: Pubkey,
        src_lst_mint_pubkey: Pubkey,
        dst_lst_mint_pubkey: Pubkey,
        src_rebalance_amount: u64,
        min_starting_src_lst: u64,
        max_starting_dst_lst: u64,
        donate_msol_from_addr: Pubkey,
        donate_msol_authority: Pubkey,
        msol_donate_amt: u64,
    ) -> Result<[Instruction; 3], Box<dyn std::error::Error>> {
        let jito_stake_pool_acc = {
            let mut test_fixtures = self.test_fixtures.lock().unwrap();
            test_fixtures
                .program_simulator
                .get_account(jito_stake_pool::id())
                .await?
                .unwrap()
        };
        let jito_sol_val_calc_accounts = SplLstSolCommonFreeArgsConst {
            spl_stake_pool: KeyedAccount {
                pubkey: jito_stake_pool::id(),
                account: jito_stake_pool_acc,
            },
        }
        .resolve_spl_to_account_metas()
        .unwrap();

        let marinade_sol_val_calc_accounts = marinade_sol_val_calc_account_metas();

        let src_lst_token_program = self.get_mint_token_program(src_lst_mint_pubkey).await?;
        let dst_lst_token_program = self.get_mint_token_program(dst_lst_mint_pubkey).await?;

        let lst_state_list_acc = {
            let mut test_fixtures = self.test_fixtures.lock().unwrap();
            test_fixtures
                .program_simulator
                .get_account(LST_STATE_LIST_ID)
                .await?
                .unwrap()
        };

        let pool_state_acc = {
            let mut test_fixtures = self.test_fixtures.lock().unwrap();
            test_fixtures
                .program_simulator
                .get_account(POOL_STATE_ID)
                .await?
                .unwrap()
        };

        let start_rebalance_args = StartRebalanceByMintsFreeArgs {
            withdraw_to: withdraw_to_pubkey,
            lst_state_list: KeyedAccount {
                pubkey: LST_STATE_LIST_ID,
                account: lst_state_list_acc,
            },
            pool_state: KeyedAccount {
                pubkey: POOL_STATE_ID,
                account: pool_state_acc,
            },
            src_lst_mint: MintWithTokenProgram {
                pubkey: src_lst_mint_pubkey,
                token_program: src_lst_token_program,
            },
            dst_lst_mint: MintWithTokenProgram {
                pubkey: dst_lst_mint_pubkey,
                token_program: dst_lst_token_program,
            },
        };

        let (
            start_rebalance_keys,
            SrcDstLstIndexes {
                src_lst_index,
                dst_lst_index,
            },
            _program_ids,
        ) = start_rebalance_args.resolve().unwrap();

        let start_rebalance_ix = start_rebalance_ix_full(
            start_rebalance_keys,
            StartRebalanceIxFullArgs {
                src_lst_index,
                dst_lst_index,
                lst_amts: StartRebalanceIxLstAmts {
                    amount: src_rebalance_amount,
                    min_starting_src_lst,
                    max_starting_dst_lst,
                },
            },
            SrcDstLstSolValueCalcAccounts {
                src_lst_calculator_program_id: spl_calculator_lib::program::ID,
                dst_lst_calculator_program_id: marinade_calculator_lib::program::ID,
                src_lst_calculator_accounts: &jito_sol_val_calc_accounts,
                dst_lst_calculator_accounts: &marinade_sol_val_calc_accounts,
            },
        )
        .unwrap();

        let end_rebalance_keys =
            EndRebalanceFromStartRebalanceKeys(&start_rebalance_keys).resolve();
        let marinade_sol_val_calc_accounts = marinade_sol_val_calc_account_metas();

        let end_rebalance_ix = end_rebalance_ix_full(
            end_rebalance_keys,
            &marinade_sol_val_calc_accounts,
            marinade_calculator_lib::program::ID,
        )
        .unwrap();

        let donate_msol_ix = transfer_checked_ix(
            TransferCheckedKeys {
                token_program: spl_token::ID,
                from: donate_msol_from_addr,
                to: end_rebalance_keys.dst_pool_reserves,
                authority: donate_msol_authority,
                mint: msol::ID,
            },
            TransferCheckedArgs {
                amount: msol_donate_amt,
                decimals: 9,
            },
        )
        .unwrap();

        Ok([start_rebalance_ix, donate_msol_ix, end_rebalance_ix])
    }

    pub async fn rebalance(
        &mut self,
        withdraw_to_pubkey: Pubkey,
        src_lst_mint_pubkey: Pubkey,
        dst_lst_mint_pubkey: Pubkey,
        src_rebalance_amount: u64,
        min_starting_src_lst: u64,
        max_starting_dst_lst: u64,
        donate_msol_from_addr: Pubkey,
        donate_msol_authority: &Keypair,
        msol_donate_amt: u64,
    ) -> TestResult {
        let instructions = self
            .rebalance_ixs(
                withdraw_to_pubkey,
                src_lst_mint_pubkey,
                dst_lst_mint_pubkey,
                src_rebalance_amount,
                min_starting_src_lst,
                max_starting_dst_lst,
                donate_msol_from_addr,
                donate_msol_authority.pubkey(),
                msol_donate_amt,
            )
            .await?;

        self.process_instructions(
            &instructions,
            &vec![&self.authority, donate_msol_authority],
            Some(&self.payer),
        )
        .await?;

        Ok(())
    }
}
