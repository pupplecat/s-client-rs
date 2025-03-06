#[cfg(test)]
mod test_set_protocol_fee_beneficiary {
    use solana_sdk::pubkey::Pubkey;
    use test_utils::{setup_s_program_test_environment, TestResult};

    #[tokio::test]
    async fn test_set_protocol_fee_beneficiary() -> TestResult {
        let mut env = setup_s_program_test_environment().await;
        env.initialize().await?;

        {
            // first time
            let new_protocol_fee_beneficiary = Pubkey::new_unique();
            env.set_protocol_fee_beneficiary(new_protocol_fee_beneficiary)
                .await?;
            let pool_state = env.get_pool_state().await?;

            assert_eq!(
                pool_state.protocol_fee_beneficiary,
                new_protocol_fee_beneficiary
            );
        }

        {
            // second time
            let new_protocol_fee_beneficiary = Pubkey::new_unique();
            env.set_protocol_fee_beneficiary(new_protocol_fee_beneficiary)
                .await?;
            let pool_state = env.get_pool_state().await?;

            assert_eq!(
                pool_state.protocol_fee_beneficiary,
                new_protocol_fee_beneficiary
            );
        }

        Ok(())
    }
}
