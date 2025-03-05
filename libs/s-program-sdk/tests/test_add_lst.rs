#[cfg(test)]
mod test_add_lst {
    use solana_sdk::signer::Signer;
    use test_utils::{setup_s_program_test_environment, TestResult};

    #[tokio::test]
    async fn test_initialize() -> TestResult {
        let mut env = setup_s_program_test_environment().await;

        env.initialize().await?;

        // env.add_lst(lst_mint_pubkey, sol_value_calculator_pubkey);

        let pool_state = env.get_pool_state().await?;

        assert_eq!(pool_state.admin, env.authority.pubkey());
        assert_eq!(pool_state.rebalance_authority, env.authority.pubkey());
        Ok(())
    }
}
