#[cfg(test)]
mod test_set_admin {
    use solana_sdk::{signature::Keypair, signer::Signer};
    use test_utils::{setup_s_program_test_environment, TestResult};

    #[tokio::test]
    async fn test_set_admin() -> TestResult {
        let mut env = setup_s_program_test_environment().await;
        env.initialize().await?;

        {
            // first time
            let new_admin_kp = Keypair::new();
            env.set_admin(&new_admin_kp).await?;
            let pool_state = env.get_pool_state().await?;

            assert_eq!(pool_state.admin, new_admin_kp.pubkey());
        }

        {
            // second time
            let new_admin_kp = Keypair::new();
            env.set_admin(&new_admin_kp).await?;
            let pool_state = env.get_pool_state().await?;

            assert_eq!(pool_state.admin, new_admin_kp.pubkey());
        }

        Ok(())
    }
}
