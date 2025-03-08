#[cfg(test)]
mod test_add_lst {

    use marinade_keys::msol;
    use test_utils::{setup_s_program_test_environment, TestResult};

    #[tokio::test]
    async fn test_add_lst() -> TestResult {
        let mut env = setup_s_program_test_environment().await;

        env.initialize().await?;

        let lst_mint = msol::ID;

        env.add_lst(lst_mint, spl_calculator_lib::program::ID)
            .await?;

        Ok(())
    }
}
