use marinade_keys::msol;
use s_controller_lib::{
    find_pool_reserves_address, find_protocol_fee_accumulator_address, FindLstPdaAtaKeys,
};
use solana_program_test::ProgramTest;
use test_utils::{
    program_test::{
        MarinadeCalculatorProgramTest, PoolStateProgramTest, SControllerProgramTest,
        SplCalculatorProgramTest, DEFAULT_POOL_STATE,
    },
    setup_s_program_test_environment_with_program_test, SProgramTestEnvironment, TestResult,
};

async fn setup() -> SProgramTestEnvironment {
    let program_test = ProgramTest::default()
        .add_s_controller_program()
        .add_spl_progs()
        .add_marinade_progs()
        .add_jito_stake_pool()
        .add_marinade_stake_pool()
        .add_pool_state(DEFAULT_POOL_STATE);

    setup_s_program_test_environment_with_program_test(program_test).await
}

#[tokio::test]
async fn test_add_lst() -> TestResult {
    let mut env = setup().await;

    let lst_mint = msol::ID;
    let lst_mint_program_id = env.get_mint_token_program(lst_mint).await?;

    let (pool_reserves, pool_reserves_bump) = find_pool_reserves_address(FindLstPdaAtaKeys {
        lst_mint,
        token_program: lst_mint_program_id,
    });

    let (protocol_fee_accumulator, protocol_fee_accumulator_bump) =
        find_protocol_fee_accumulator_address(FindLstPdaAtaKeys {
            lst_mint,
            token_program: lst_mint_program_id,
        });

    let lst_state_list = env.get_lst_state_list().await?;
    assert_eq!(lst_state_list, None);

    env.add_lst(lst_mint, marinade_calculator_lib::program::ID)
        .await?;

    let lst_state_list = env.get_lst_state_list().await?;
    let pool_reserves_token_account = env.get_token_account_by_pubkey(&pool_reserves).await?;
    let protocol_fee_accumulator_token_account = env
        .get_token_account_by_pubkey(&protocol_fee_accumulator)
        .await?;

    // verify lst state list account
    assert!(lst_state_list.is_some());
    let lst_state_list = lst_state_list.unwrap();
    assert_eq!(lst_state_list.len(), 1);
    assert_eq!(
        lst_state_list[0].sol_value_calculator,
        marinade_calculator_lib::program::ID
    );
    assert_eq!(lst_state_list[0].mint, lst_mint);
    assert_eq!(lst_state_list[0].is_input_disabled, 0);
    assert_eq!(lst_state_list[0].pool_reserves_bump, pool_reserves_bump);
    assert_eq!(
        lst_state_list[0].protocol_fee_accumulator_bump,
        protocol_fee_accumulator_bump
    );
    assert_eq!(lst_state_list[0].padding, [0; 5]);
    assert_eq!(lst_state_list[0].sol_value, 0);

    // verify token accounts
    assert_eq!(pool_reserves_token_account.mint, lst_mint);
    assert_eq!(
        pool_reserves_token_account.owner,
        env.get_pool_state_pubkey()
    );
    assert_eq!(protocol_fee_accumulator_token_account.mint, lst_mint);
    assert_eq!(
        protocol_fee_accumulator_token_account.owner,
        env.get_protocol_fee_auth_pubkey()
    );

    Ok(())
}
