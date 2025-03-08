use solana_program_test::ProgramTest;
use solana_sdk::pubkey::Pubkey;
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
async fn test_add_disable_pool_authority() -> TestResult {
    let mut env = setup().await;

    let authority_list = env.get_disable_pool_authority_list().await?;

    assert_eq!(authority_list, None);

    let new_authority_1 = Pubkey::new_unique();
    let new_authority_2 = Pubkey::new_unique();
    env.add_disable_pool_authority(new_authority_1).await?;
    env.add_disable_pool_authority(new_authority_2).await?;

    let authority_list = env.get_disable_pool_authority_list().await?;

    assert!(authority_list.is_some());
    let authority_list = authority_list.unwrap();
    assert_eq!(authority_list.len(), 2);
    assert_eq!(authority_list[0], new_authority_1);
    assert_eq!(authority_list[1], new_authority_2);

    Ok(())
}
