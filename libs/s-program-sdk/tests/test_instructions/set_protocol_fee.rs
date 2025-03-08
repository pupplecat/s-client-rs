#[cfg(test)]
mod test_set_protocol_fee {
    use test_utils::{setup_s_program_test_environment, TestResult};

    #[tokio::test]
    async fn test_set_protocol_fee() -> TestResult {
        let mut env = setup_s_program_test_environment().await;
        env.initialize().await?;

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
}
