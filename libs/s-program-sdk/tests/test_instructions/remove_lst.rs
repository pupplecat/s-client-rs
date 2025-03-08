use marinade_keys::msol;
use solana_program_test::ProgramTest;
use test_utils::{
    program_test::{
        MarinadeCalculatorProgramTest, PoolStateProgramTest, SControllerProgramTest,
        SplCalculatorProgramTest, DEFAULT_POOL_STATE,
    },
    setup_s_program_test_environment_with_program_test,
    utils::jitosol,
    SProgramTestEnvironment, TestResult,
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
async fn test_remove_lst() -> TestResult {
    let mut env = setup().await;

    let lst_mint = msol::ID;
    env.add_lst(lst_mint, spl_calculator_lib::program::ID)
        .await?;

    env.remove_lst(lst_mint).await?;

    let lst_state_list = env.get_lst_state_list().await?;
    assert_eq!(lst_state_list, None);

    Ok(())
}

#[tokio::test]
async fn test_remove_lst_2() -> TestResult {
    let mut env = setup().await;

    let lst_mint_1 = msol::ID;
    env.add_lst(lst_mint_1, marinade_calculator_lib::program::ID)
        .await?;
    let lst_mint_2 = jitosol::id();
    env.add_lst(lst_mint_2, spl_calculator_lib::program::ID)
        .await?;

    env.remove_lst(lst_mint_1).await?;

    let lst_state_list = env.get_lst_state_list().await?;
    assert!(lst_state_list.is_some());

    let lst_state_list = lst_state_list.unwrap();
    assert_eq!(lst_state_list.len(), 1);
    assert_eq!(lst_state_list[0].mint, lst_mint_2);

    Ok(())
}
