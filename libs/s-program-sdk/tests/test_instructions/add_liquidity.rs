use marinade_keys::msol;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};
use test_utils::{
    program_test::{GenAndAddTokenAccountProgramTest, SControllerProgramTest},
    setup_s_program_test_environment_with_program_test,
    utils::{jitosol, MockTokenAccountAtaArgs, JITO_STAKE_POOL_LAST_UPDATE_EPOCH},
    SProgramTestEnvironment, TestResult,
};

use super::common::{jito_marinade_no_fee_program_test, JitoMarinadeProgramTestArgs};

const JITOSOL_TO_ADD: u64 = 1_000_000_000;
const MSOL_TO_ADD: u64 = 1_000_000_000;

async fn setup() -> (SProgramTestEnvironment, Keypair) {
    let liquidity_provider = Keypair::new();
    let lp_token_mint = Pubkey::new_unique();

    let mut program_test = jito_marinade_no_fee_program_test(JitoMarinadeProgramTestArgs {
        jitosol_sol_value: 0,
        msol_sol_value: 0,
        jitosol_reserves: 0,
        msol_reserves: 0,
        jitosol_protocol_fee_accumulator: 0,
        msol_protocol_fee_accumulator: 0,
        lp_token_mint,
        lp_token_supply: 0,
    })
    .add_s_controller_program();

    program_test.gen_ata_and_add_token_account(MockTokenAccountAtaArgs {
        mint: jitosol::id(),
        authority: liquidity_provider.pubkey(),
        amount: JITOSOL_TO_ADD,
        token_program_id: spl_token::ID,
    });
    program_test.gen_ata_and_add_token_account(MockTokenAccountAtaArgs {
        mint: msol::ID,
        authority: liquidity_provider.pubkey(),
        amount: MSOL_TO_ADD,
        token_program_id: spl_token::ID,
    });
    program_test.gen_ata_and_add_token_account(MockTokenAccountAtaArgs {
        mint: lp_token_mint,
        authority: liquidity_provider.pubkey(),
        amount: MSOL_TO_ADD,
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
async fn test_add_liquidity() -> TestResult {
    let (mut env, liquidity_provider) = setup().await;

    let lst_mint_pubkey = msol::ID;

    env.add_liquidity(lst_mint_pubkey, MSOL_TO_ADD, liquidity_provider)
        .await?;

    Ok(())
}
