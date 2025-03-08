#[cfg(test)]
mod test_set_sol_value_calculator {

    use marinade_keys::{marinade_state, msol};
    use test_utils::{setup_s_program_test_environment, TestResult};

    #[tokio::test]
    async fn test_set_sol_value_calculator() -> TestResult {
        let mut env = setup_s_program_test_environment().await;

        env.initialize().await?;
        env.set_pricing_program(no_fee_pricing_program::ID).await?;

        env.add_lst(msol::ID, marinade_calculator_lib::program::ID)
            .await?;

        println!("xxx marinade_state::ID {}", marinade_state::ID);
        let lst_mint = msol::ID;
        env.set_sol_value_calculator(lst_mint).await?;

        Ok(())
    }
}
