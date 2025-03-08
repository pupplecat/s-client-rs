#[cfg(test)]
mod test_set_pricing_program {
    use test_utils::{setup_s_program_test_environment, TestResult};

    #[tokio::test]
    async fn test_set_pricing_program() -> TestResult {
        let mut env = setup_s_program_test_environment().await;
        env.initialize().await?;

        let new_pricing_program_id = no_fee_pricing_program::ID;
        env.set_pricing_program(new_pricing_program_id).await?;
        let pool_state = env.get_pool_state().await?;

        assert_eq!(pool_state.pricing_program, new_pricing_program_id);

        Ok(())
    }
}
