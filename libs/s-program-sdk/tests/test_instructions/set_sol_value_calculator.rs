use marinade_keys::msol;
use solana_sdk::pubkey::Pubkey;
use test_utils::{
    program_test::{LstStateListProgramTest, MockLstStateArgs, SControllerProgramTest},
    setup_s_program_test_environment_with_program_test,
    utils::try_find_lst_state_index,
    SProgramTestEnvironment, TestResult,
};

use crate::test_instructions::common::{
    jito_marinade_no_fee_program_test, JitoMarinadeProgramTestArgs,
};

const MSOL_POOL_RESERVES: u64 = 1_000_000_000;

async fn setup() -> SProgramTestEnvironment {
    let program_test = jito_marinade_no_fee_program_test(JitoMarinadeProgramTestArgs {
        // these are overriden below
        msol_reserves: MSOL_POOL_RESERVES,
        msol_sol_value: MSOL_POOL_RESERVES,
        // dont cares
        jitosol_reserves: 0,
        jitosol_sol_value: 0,
        jitosol_protocol_fee_accumulator: 0,
        msol_protocol_fee_accumulator: 0,
        lp_token_mint: Pubkey::new_unique(),
        lp_token_supply: 0,
    })
    .add_mock_lst_states(
        // set mSOL initial calculator to a broken pubkey
        &[MockLstStateArgs {
            mint: msol::ID,
            sol_value_calculator: Pubkey::new_unique(),
            token_program: spl_token::ID,
            sol_value: MSOL_POOL_RESERVES,
            reserves_amt: MSOL_POOL_RESERVES,
            protocol_fee_accumulator_amt: 0,
            is_input_disabled: false,
        }],
    )
    .add_s_controller_program();

    setup_s_program_test_environment_with_program_test(program_test).await
}

#[tokio::test]
async fn test_set_sol_value_calculator() -> TestResult {
    let mut env = setup().await;

    let lst_mint_pubkey = msol::ID;

    let lst_state = {
        let lst_state_list = env.get_lst_state_list().await?.unwrap_or_default();
        let (_, lst_state) = try_find_lst_state_index(&lst_state_list, lst_mint_pubkey)?;

        lst_state.clone()
    };

    assert!(lst_state.sol_value_calculator != marinade_calculator_lib::program::ID);

    env.set_sol_value_calculator(lst_mint_pubkey).await?;

    let lst_state = {
        let lst_state_list = env.get_lst_state_list().await?.unwrap_or_default();
        let (_, lst_state) = try_find_lst_state_index(&lst_state_list, lst_mint_pubkey)?;

        lst_state.clone()
    };

    assert_eq!(
        lst_state.sol_value_calculator,
        marinade_calculator_lib::program::ID
    );
    assert!(lst_state.sol_value > MSOL_POOL_RESERVES); // should have increased to true rate after sync

    let pool_state = env.get_pool_state().await?;
    assert!(pool_state.total_sol_value > MSOL_POOL_RESERVES); // should have increased to true rate after sync

    Ok(())
}
