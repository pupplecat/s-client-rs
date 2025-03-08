use marinade_keys::msol;
use s_controller_interface::PoolState;
use solana_program_test::ProgramTest;
use solana_sdk::pubkey::Pubkey;
use test_utils::{
    program_test::{
        FlatFeeProgramTest, LpTokenProgramTest, LstStateListProgramTest,
        MarinadeCalculatorProgramTest, MockLstStateArgs, MockPoolState, SplCalculatorProgramTest,
        DEFAULT_POOL_STATE,
    },
    utils::{jitosol, IntoAccount},
};

#[derive(Clone, Copy, Default, Debug)]
pub struct JitoMarinadeProgramTestArgs {
    pub jitosol_sol_value: u64,
    pub msol_sol_value: u64,
    pub jitosol_reserves: u64,
    pub msol_reserves: u64,
    pub jitosol_protocol_fee_accumulator: u64,
    pub msol_protocol_fee_accumulator: u64,
    pub lp_token_mint: Pubkey,
    pub lp_token_supply: u64,
}

impl JitoMarinadeProgramTestArgs {
    pub fn with_lp_token_mint(mut self, lp_token_mint: Pubkey) -> Self {
        self.lp_token_mint = lp_token_mint;
        self
    }
}

pub fn jito_marinade_base_program_test(
    JitoMarinadeProgramTestArgs {
        jitosol_sol_value,
        msol_sol_value,
        jitosol_reserves,
        msol_reserves,
        jitosol_protocol_fee_accumulator,
        msol_protocol_fee_accumulator,
        lp_token_mint,
        lp_token_supply,
    }: JitoMarinadeProgramTestArgs,
) -> (ProgramTest, PoolState) {
    let mut program_test = ProgramTest::default();
    program_test = program_test
        .add_spl_progs()
        .add_marinade_progs()
        .add_jito_stake_pool()
        .add_marinade_stake_pool()
        .add_mock_lst_states(&[
            MockLstStateArgs {
                mint: jitosol::id(),
                sol_value: jitosol_sol_value,
                reserves_amt: jitosol_reserves,
                protocol_fee_accumulator_amt: jitosol_protocol_fee_accumulator,
                token_program: spl_token::ID,
                sol_value_calculator: spl_calculator_lib::program::ID,
                is_input_disabled: false,
            },
            MockLstStateArgs {
                mint: msol::ID,
                sol_value: msol_sol_value,
                reserves_amt: msol_reserves,
                protocol_fee_accumulator_amt: msol_protocol_fee_accumulator,
                token_program: spl_token::ID,
                sol_value_calculator: marinade_calculator_lib::program::ID,
                is_input_disabled: false,
            },
        ])
        .add_mock_lp_mint(lp_token_mint, lp_token_supply);

    let total_sol_value = jitosol_sol_value + msol_sol_value;

    let mut pool_state = DEFAULT_POOL_STATE;
    pool_state.total_sol_value = total_sol_value;
    pool_state.lp_token_mint = lp_token_mint;

    (program_test, pool_state)
}

pub fn jito_marinade_no_fee_program_test(args: JitoMarinadeProgramTestArgs) -> ProgramTest {
    let (mut program_test, pool_state) = jito_marinade_base_program_test(args);

    program_test.add_account(
        s_controller_lib::program::POOL_STATE_ID,
        MockPoolState(PoolState {
            pricing_program: no_fee_pricing_program::ID,
            ..pool_state
        })
        .into_account(),
    );

    program_test.add_no_fee_program()
}
