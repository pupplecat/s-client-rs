#[cfg(test)]
mod setup {
    use test_utils::{setup_s_program_test_environment, TestResult};

    #[tokio::test]
    async fn test_setup() -> TestResult {
        let mut env = setup_s_program_test_environment().await;

        env.initialize().await?;
        Ok(())
    }
}
