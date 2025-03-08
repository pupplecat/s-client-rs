use marinade_keys::msol;
use solana_program_test::ProgramTest;
use test_utils::{
    program_test::{PoolStateProgramTest, SControllerProgramTest, DEFAULT_POOL_STATE},
    setup_s_program_test_environment_with_program_test, SProgramTestEnvironment, TestResult,
};

async fn setup() -> SProgramTestEnvironment {
    let program_test = ProgramTest::default()
        .add_s_controller_program()
        .add_pool_state(DEFAULT_POOL_STATE);

    setup_s_program_test_environment_with_program_test(program_test).await
}
#[tokio::test]
async fn test_add_lst() -> TestResult {
    let mut env = setup().await;

    let lst_mint = msol::ID;

    env.add_lst(lst_mint, spl_calculator_lib::program::ID)
        .await?;

    Ok(())
}
