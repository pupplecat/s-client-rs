use solana_program_test::ProgramTest;
use solana_sdk::{signature::Keypair, signer::Signer};
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
async fn test_set_protocol_fee_beneficiary() -> TestResult {
    let mut env = setup().await;

    {
        // first time
        let new_protocol_fee_beneficiary = Keypair::new();
        env.set_protocol_fee_beneficiary(&new_protocol_fee_beneficiary)
            .await?;
        let pool_state = env.get_pool_state().await?;

        assert_eq!(
            pool_state.protocol_fee_beneficiary,
            new_protocol_fee_beneficiary.pubkey()
        );
    }

    {
        // second time
        let new_protocol_fee_beneficiary = Keypair::new();
        env.set_protocol_fee_beneficiary(&new_protocol_fee_beneficiary)
            .await?;
        let pool_state = env.get_pool_state().await?;

        assert_eq!(
            pool_state.protocol_fee_beneficiary,
            new_protocol_fee_beneficiary.pubkey()
        );
    }

    Ok(())
}
