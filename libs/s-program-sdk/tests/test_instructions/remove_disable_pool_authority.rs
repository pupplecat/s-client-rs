#[cfg(test)]
mod test_remove_disable_pool_authority {
    use solana_sdk::pubkey::Pubkey;
    use test_utils::{setup_s_program_test_environment, TestResult};

    #[tokio::test]
    async fn test_remove_disable_pool_authority() -> TestResult {
        let mut env = setup_s_program_test_environment().await;

        env.initialize().await?;
        let new_authority = Pubkey::new_unique();
        env.add_disable_pool_authority(new_authority).await?;

        env.remove_disable_pool_authority(new_authority).await?;

        let authority_list = env.get_disable_pool_authority_list().await?;
        assert!(authority_list.is_none());
        Ok(())
    }
}
