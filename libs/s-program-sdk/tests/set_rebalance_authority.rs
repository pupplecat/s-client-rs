#[cfg(test)]
mod test_set_rebalance_authority {
    use solana_sdk::{signature::Keypair, signer::Signer};
    use test_utils::{setup_s_program_test_environment, TestResult};

    #[tokio::test]
    async fn test_set_rebalance_authority() -> TestResult {
        let mut env = setup_s_program_test_environment().await;
        env.initialize().await?;

        {
            // first time
            let new_rebalance_authority = Keypair::new();
            env.set_rebalance_authority(&new_rebalance_authority)
                .await?;
            let pool_state = env.get_pool_state().await?;

            assert_eq!(
                pool_state.rebalance_authority,
                new_rebalance_authority.pubkey()
            );
        }

        {
            // second time
            let new_rebalance_authority = Keypair::new();
            env.set_rebalance_authority(&new_rebalance_authority)
                .await?;
            let pool_state = env.get_pool_state().await?;

            assert_eq!(
                pool_state.rebalance_authority,
                new_rebalance_authority.pubkey()
            );
        }

        Ok(())
    }
}
