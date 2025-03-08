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
async fn test_set_protocol_fee() -> TestResult {
    let mut env = setup().await;

    {
        // first time
        let new_trading_protocol_fee_bps = Some(10);
        let new_lp_protocol_fee_bps = Some(20);
        env.set_protocol_fee(new_trading_protocol_fee_bps, new_lp_protocol_fee_bps)
            .await?;
        let pool_state = env.get_pool_state().await?;

        assert_eq!(
            pool_state.trading_protocol_fee_bps,
            new_trading_protocol_fee_bps.unwrap()
        );
        assert_eq!(
            pool_state.lp_protocol_fee_bps,
            new_lp_protocol_fee_bps.unwrap()
        );
    }

    {
        // second time
        let new_trading_protocol_fee_bps = Some(30);
        let new_lp_protocol_fee_bps = None;
        env.set_protocol_fee(new_trading_protocol_fee_bps, new_lp_protocol_fee_bps)
            .await?;
        let pool_state = env.get_pool_state().await?;

        assert_eq!(
            pool_state.trading_protocol_fee_bps,
            new_trading_protocol_fee_bps.unwrap()
        );
        assert_eq!(pool_state.lp_protocol_fee_bps, 20);
    }

    Ok(())
}
