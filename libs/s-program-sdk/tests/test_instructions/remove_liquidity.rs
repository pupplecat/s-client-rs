use marinade_keys::msol;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};
use spl_associated_token_account::get_associated_token_address_with_program_id;
use test_utils::{
    program_test::{GenAndAddTokenAccountProgramTest, SControllerProgramTest},
    setup_s_program_test_environment_with_program_test,
    utils::{jitosol, MockTokenAccountAtaArgs, JITO_STAKE_POOL_LAST_UPDATE_EPOCH},
    SProgramTestEnvironment, TestResult,
};

use super::common::{jito_marinade_no_fee_program_test, JitoMarinadeProgramTestArgs};

const LP_TOKEN_SUPPLY: u64 = 1_000_000_000;
const LP_TOKENS_TO_REMOVE: u64 = LP_TOKEN_SUPPLY;
const MSOL_RESERVES_STARTING_BALANCE: u64 = 1_000_000_000;

async fn setup() -> (SProgramTestEnvironment, Keypair) {
    let liquidity_provider = Keypair::new();
    let lp_token_mint = Pubkey::new_unique();

    let mut program_test = jito_marinade_no_fee_program_test(JitoMarinadeProgramTestArgs {
        jitosol_sol_value: 0, // will increase on SyncSolValue
        jitosol_reserves: 0,
        lp_token_mint,
        lp_token_supply: LP_TOKEN_SUPPLY,
        msol_sol_value: MSOL_RESERVES_STARTING_BALANCE,
        msol_reserves: MSOL_RESERVES_STARTING_BALANCE,
        jitosol_protocol_fee_accumulator: 0,
        msol_protocol_fee_accumulator: 0,
    })
    .add_s_controller_program();

    program_test.gen_ata_and_add_token_account(MockTokenAccountAtaArgs {
        mint: jitosol::id(),
        authority: liquidity_provider.pubkey(),
        amount: 0,
        token_program_id: spl_token::ID,
    });
    program_test.gen_ata_and_add_token_account(MockTokenAccountAtaArgs {
        mint: msol::ID,
        authority: liquidity_provider.pubkey(),
        amount: 0,
        token_program_id: spl_token::ID,
    });
    program_test.gen_ata_and_add_token_account(MockTokenAccountAtaArgs {
        mint: lp_token_mint,
        authority: liquidity_provider.pubkey(),
        amount: LP_TOKENS_TO_REMOVE,
        token_program_id: spl_token::ID,
    });

    let env = setup_s_program_test_environment_with_program_test(program_test).await;

    {
        let mut test_fixture = env.test_fixtures.lock().unwrap();
        test_fixture
            .program_simulator
            .warp_to_epoch(JITO_STAKE_POOL_LAST_UPDATE_EPOCH)
            .unwrap();
    }

    (env, liquidity_provider)
}

#[tokio::test]
async fn test_remove_liquidity() -> TestResult {
    let (mut env, liquidity_provider) = setup().await;

    let lst_mint_pubkey = msol::ID;
    let liquidity_provider_address = liquidity_provider.pubkey();

    env.remove_liquidity(lst_mint_pubkey, LP_TOKENS_TO_REMOVE, liquidity_provider)
        .await?;

    let lp_balance = {
        let pool_state = env.get_pool_state().await?;
        let dst_lp_acc = get_associated_token_address_with_program_id(
            &liquidity_provider_address,
            &pool_state.lp_token_mint,
            &spl_token::ID,
        );

        let mut test_fixutres = env.test_fixtures.lock().unwrap();
        test_fixutres.balance_of_token_account(&dst_lp_acc).await?
    };

    let msol_balance = {
        let dst_lp_acc = get_associated_token_address_with_program_id(
            &liquidity_provider_address,
            &msol::ID,
            &spl_token::ID,
        );

        let mut test_fixutres = env.test_fixtures.lock().unwrap();
        test_fixutres.balance_of_token_account(&dst_lp_acc).await?
    };

    assert_eq!(lp_balance, 0);
    assert!(msol_balance > 0);

    Ok(())
}
