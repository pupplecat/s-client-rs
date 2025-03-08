#[cfg(test)]
mod test_remove_lst {

    use marinade_keys::msol;
    use test_utils::{setup_s_program_test_environment, TestResult};

    #[tokio::test]
    async fn test_remove_lst() -> TestResult {
        let mut env = setup_s_program_test_environment().await;

        env.initialize().await?;

        let lst_mint = msol::ID;
        env.add_lst(lst_mint, spl_calculator_lib::program::ID)
            .await?;

        env.remove_lst(lst_mint).await?;

        let lst_state_list = env.get_lst_state_list().await?;
        assert_eq!(lst_state_list, None);

        Ok(())
    }
}
