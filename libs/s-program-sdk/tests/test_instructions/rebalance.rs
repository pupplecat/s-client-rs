use std::u64;

use marinade_keys::msol;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};
use test_utils::{
    program_test::{GenAndAddTokenAccountProgramTest, SControllerProgramTest},
    setup_s_program_test_environment_with_program_test,
    utils::{jitosol, MockTokenAccountAtaArgs},
    SProgramTestEnvironment, TestResult,
};

use super::common::{jito_marinade_no_fee_program_test, JitoMarinadeProgramTestArgs};

const JITOSOL_START_SOL_VALUE: u64 = 1_000_000_000;
const MSOL_START_SOL_VALUE: u64 = 1_000_000_000;
const JITOSOL_WITHDRAW_AMT: u64 = 500_000_000;
const MSOL_DONATE_AMT: u64 = 500_000_000;

async fn setup() -> (SProgramTestEnvironment, Keypair, Pubkey, Pubkey) {
    let donor = Keypair::new();

    let mut program_test = jito_marinade_no_fee_program_test(JitoMarinadeProgramTestArgs {
        jitosol_sol_value: JITOSOL_START_SOL_VALUE,
        msol_sol_value: MSOL_START_SOL_VALUE,
        jitosol_reserves: JITOSOL_START_SOL_VALUE,
        msol_reserves: MSOL_START_SOL_VALUE,
        jitosol_protocol_fee_accumulator: 0,
        msol_protocol_fee_accumulator: 0,
        lp_token_mint: Pubkey::new_unique(),
        lp_token_supply: 0,
    })
    .add_s_controller_program();

    let donor_jitosol_withdraw_to =
        program_test.gen_ata_and_add_token_account(MockTokenAccountAtaArgs {
            mint: jitosol::id(),
            authority: donor.pubkey(),
            amount: 0,
            token_program_id: spl_token::ID,
        });

    let donor_msol_from = program_test.gen_ata_and_add_token_account(MockTokenAccountAtaArgs {
        mint: msol::ID,
        authority: donor.pubkey(),
        amount: MSOL_DONATE_AMT,
        token_program_id: spl_token::ID,
    });

    (
        setup_s_program_test_environment_with_program_test(program_test).await,
        donor,
        donor_jitosol_withdraw_to,
        donor_msol_from,
    )
}

#[tokio::test]
pub async fn test_rebalance() -> TestResult {
    let (mut env, donor, donor_jitosol_withdraw_to, donor_msol_from) = setup().await;
    println!("xxx {donor_jitosol_withdraw_to:?}");
    env.rebalance(
        donor_jitosol_withdraw_to,
        jitosol::id(),
        msol::ID,
        JITOSOL_WITHDRAW_AMT,
        0,
        u64::MAX,
        donor_msol_from,
        &donor,
        MSOL_DONATE_AMT,
    )
    .await?;

    Ok(())
}
