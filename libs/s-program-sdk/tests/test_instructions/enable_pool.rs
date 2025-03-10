use s_controller_interface::PoolState;
use solana_program_test::ProgramTest;
use solana_sdk::{signature::Keypair, signer::Signer};
use test_utils::{
    program_test::{PoolStateProgramTest, SControllerProgramTest, DEFAULT_POOL_STATE},
    setup_s_program_test_environment_with_program_test, SProgramTestEnvironment, TestResult,
};

async fn setup() -> SProgramTestEnvironment {
    let program_test = ProgramTest::default()
        .add_s_controller_program()
        .add_pool_state(PoolState {
            is_disabled: 1,
            ..DEFAULT_POOL_STATE
        });

    setup_s_program_test_environment_with_program_test(program_test).await
}

#[tokio::test]
async fn test_enable_pool() -> TestResult {
    let mut env = setup().await;

    {
        env.enable_pool().await?;
        let pool_state = env.get_pool_state().await?;

        assert_eq!(pool_state.is_disabled, 0);
    }

    Ok(())
}
