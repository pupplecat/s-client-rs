#[cfg(test)]
mod test_add_lst {

    use s_controller_lib::{
        find_pool_reserves_address, find_protocol_fee_accumulator_address, FindLstPdaAtaKeys,
    };
    use solana_sdk::signer::Signer;
    use test_utils::{setup_s_program_test_environment, TestResult};

    #[tokio::test]
    async fn test_add_lst() -> TestResult {
        let mut env = setup_s_program_test_environment().await;

        env.initialize().await?;

        let lst_mint = {
            let mut test_fixtures = env.test_fixtures.lock().unwrap();
            test_fixtures.create_mint(&env.payer.pubkey(), 9).await?
        };

        let (_pool_reserves, pool_reserves_bump) = find_pool_reserves_address(FindLstPdaAtaKeys {
            lst_mint,
            token_program: spl_token::ID,
        });

        let (_protocol_fee_accumulator, protocol_fee_accumulator_bump) =
            find_protocol_fee_accumulator_address(FindLstPdaAtaKeys {
                lst_mint,
                token_program: spl_token::ID,
            });

        let lst_state_list = env.get_lst_state_list().await?;
        assert_eq!(lst_state_list, None);

        env.add_lst(lst_mint, spl_calculator_lib::program::ID)
            .await?;

        let lst_state_list = env.get_lst_state_list().await?;

        assert!(lst_state_list.is_some());
        let lst_state_list = lst_state_list.unwrap();
        assert_eq!(lst_state_list.len(), 1);
        assert_eq!(
            lst_state_list[0].sol_value_calculator,
            spl_calculator_lib::program::ID
        );
        assert_eq!(lst_state_list[0].mint, lst_mint);
        assert_eq!(lst_state_list[0].is_input_disabled, 0);
        assert_eq!(lst_state_list[0].pool_reserves_bump, pool_reserves_bump);
        assert_eq!(
            lst_state_list[0].protocol_fee_accumulator_bump,
            protocol_fee_accumulator_bump
        );
        assert_eq!(lst_state_list[0].padding, [0; 5]);
        assert_eq!(lst_state_list[0].sol_value, 0);

        Ok(())
    }
}
