use marinade_keys::msol;
use solana_sdk::pubkey::Pubkey;
use test_utils::{
    program_test::SControllerProgramTest, setup_s_program_test_environment_with_program_test,
    utils::try_find_lst_state_index, SProgramTestEnvironment, TestResult,
};

use crate::test_instructions::common::{
    jito_marinade_no_fee_program_test, JitoMarinadeProgramTestArgs,
};

async fn setup() -> SProgramTestEnvironment {
    let program_test = jito_marinade_no_fee_program_test(
        JitoMarinadeProgramTestArgs::default().with_lp_token_mint(Pubkey::new_unique()),
    )
    .add_s_controller_program();

    setup_s_program_test_environment_with_program_test(program_test).await
}

#[tokio::test]
async fn test_enable_disable_lst_input() -> TestResult {
    let mut env = setup().await;

    let lst_mint_pubkey = msol::ID;

    let lst_state = {
        let lst_state_list = env.get_lst_state_list().await?.unwrap_or_default();
        let (_, lst_state) = try_find_lst_state_index(&lst_state_list, lst_mint_pubkey)?;

        lst_state.clone()
    };

    assert_eq!(lst_state.is_input_disabled, 0);

    env.disable_lst_input(lst_mint_pubkey).await?;

    {
        let lst_state = {
            let lst_state_list = env.get_lst_state_list().await?.unwrap_or_default();
            let (_, lst_state) = try_find_lst_state_index(&lst_state_list, lst_mint_pubkey)?;

            lst_state.clone()
        };

        assert_eq!(lst_state.is_input_disabled, 1);
    }

    env.enable_lst_input(lst_mint_pubkey).await?;

    {
        let lst_state = {
            let lst_state_list = env.get_lst_state_list().await?.unwrap_or_default();
            let (_, lst_state) = try_find_lst_state_index(&lst_state_list, lst_mint_pubkey)?;

            lst_state.clone()
        };

        assert_eq!(lst_state.is_input_disabled, 0);
    }
    Ok(())
}
