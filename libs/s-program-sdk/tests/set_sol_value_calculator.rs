#[cfg(test)]
mod test_set_sol_value_calculator {

    use std::str::FromStr;

    use marinade_keys::{marinade_state, msol};
    use solana_sdk::pubkey::Pubkey;
    use test_utils::{setup_s_program_test_environment, TestResult};

    #[tokio::test]
    async fn test_set_sol_value_calculator() -> TestResult {
        let mut env = setup_s_program_test_environment().await;

        // let jitosol_id: Pubkey =
        //     Pubkey::from_str("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn").unwrap();

        env.initialize().await?;
        env.set_pricing_program(no_fee_pricing_program::ID).await?;

        env.add_lst(msol::ID, marinade_calculator_lib::program::ID)
            .await?;

        // env.add_lst(jitosol_id, spl_calculator_lib::program::ID)
        //     .await?;

        println!("xxx marinade_state::ID {}", marinade_state::ID);
        // println!(
        //     "xxx state {}",
        //     Pubkey::find_program_address(&[b"state"], &marinade_calculator_lib::program::ID).0
        // );
        let lst_mint = msol::ID;
        env.set_sol_value_calculator(lst_mint).await?;

        Ok(())
    }
}
