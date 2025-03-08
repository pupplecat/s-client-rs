use solana_program_test::ProgramTest;
use test_utils::{
    program_test::{
        FlatFeeProgramTest, PoolStateProgramTest, SControllerProgramTest, DEFAULT_POOL_STATE,
    },
    setup_s_program_test_environment_with_program_test, SProgramTestEnvironment, TestResult,
};

async fn setup() -> SProgramTestEnvironment {
    let program_test = ProgramTest::default()
        .add_s_controller_program()
        .add_pool_state(DEFAULT_POOL_STATE)
        .add_no_fee_program();

    setup_s_program_test_environment_with_program_test(program_test).await
}

#[tokio::test]
async fn test_set_pricing_program() -> TestResult {
    let mut env = setup().await;

    let new_pricing_program_id = no_fee_pricing_program::ID;
    env.set_pricing_program(new_pricing_program_id).await?;
    let pool_state = env.get_pool_state().await?;

    assert_eq!(pool_state.pricing_program, new_pricing_program_id);

    Ok(())
}
