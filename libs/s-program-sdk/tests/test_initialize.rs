#[cfg(test)]
mod setup {
    use solana_sdk::signer::Signer;
    use test_utils::{setup_s_program_test_environment, TestResult};

    #[tokio::test]
    async fn test_setup() -> TestResult {
        let mut env = setup_s_program_test_environment().await;

        env.initialize().await?;

        let pool_state = env.get_pool_state().await?;

        assert_eq!(pool_state.admin, env.authority.pubkey());
        assert_eq!(pool_state.rebalance_authority, env.authority.pubkey());
        Ok(())
    }
}
