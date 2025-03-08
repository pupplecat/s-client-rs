#[cfg(test)]
mod test_initialize {
    use solana_sdk::{program_option::COption, signer::Signer};
    use test_utils::{setup_s_program_test_environment, TestResult};

    #[tokio::test]
    pub async fn test_initialize() -> TestResult {
        let mut env = setup_s_program_test_environment().await;

        env.initialize().await?;

        let pool_state = env.get_pool_state().await?;

        assert_eq!(pool_state.total_sol_value, 0);
        assert_eq!(pool_state.trading_protocol_fee_bps, 1_000); // DEFAULT_TRADING_PROTOCOL_FEE_BPS (10%)
        assert_eq!(pool_state.lp_protocol_fee_bps, 1_000); // DEFAULT_LP_PROTOCOL_FEE_BPS (10%)
        assert_eq!(pool_state.version, 1); // CURRENT_PROGRAM_VERS
        assert_eq!(pool_state.is_disabled, 0);
        assert_eq!(pool_state.is_rebalancing, 0);
        assert_eq!(pool_state.admin, env.authority.pubkey()); // initial_authority::ID
        assert_eq!(pool_state.rebalance_authority, env.authority.pubkey()); // initial_authority::ID
        assert_eq!(pool_state.protocol_fee_beneficiary, env.authority.pubkey()); // initial_authority::ID
        assert_eq!(pool_state.pricing_program, flat_fee_lib::program::ID); // DEFAULT_PRICING_PROGRAM
        assert_eq!(pool_state.lp_token_mint, env.lp_mint);
        assert_eq!(pool_state.padding, [0; 1]); // don't care

        let lp_mint = env.get_mint_account(pool_state.lp_token_mint).await?;
        assert_eq!(
            lp_mint.mint_authority,
            COption::Some(env.get_pool_state_pubkey())
        );
        assert_eq!(lp_mint.supply, 0);
        assert_eq!(lp_mint.decimals, 9); // native_mint::DECIMALS,
        assert_eq!(lp_mint.is_initialized, true);
        assert_eq!(
            lp_mint.freeze_authority,
            COption::Some(env.get_pool_state_pubkey())
        );
        Ok(())
    }
}
