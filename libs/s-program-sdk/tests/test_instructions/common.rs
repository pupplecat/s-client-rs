use lido_keys::stsol;
use marinade_keys::msol;
use s_controller_interface::PoolState;
use solana_program_test::ProgramTest;
use solana_sdk::pubkey::Pubkey;
use spl_token::native_mint;
use test_utils::{
    program_test::{
        FlatFeeProgramTest, LidoCalculatorProgramTest, LpTokenProgramTest, LstStateListProgramTest,
        MarinadeCalculatorProgramTest, MockLstStateArgs, MockPoolState, MockProgramState,
        MockProtocolFeeBps, SplCalculatorProgramTest, WsolCalculatorProgramTest,
        DEFAULT_POOL_STATE,
    },
    utils::{jitosol, IntoAccount, MockFeeAccount, MockFeeAccountArgs},
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

#[derive(Clone, Copy, Default, Debug)]
pub struct LidoWsolProgramTestArgs {
    pub wsol_reserves: u64,
    pub stsol_sol_value: u64,
    pub stsol_reserves: u64,
    pub wsol_protocol_fee_accumulator: u64,
    pub stsol_protocol_fee_accumulator: u64,
    pub lp_token_mint: Pubkey,
    pub lp_token_supply: u64,
}

pub fn lido_wsol_flat_fee_program_test(
    args: LidoWsolProgramTestArgs,
    flat_fee_state: flat_fee_interface::ProgramState,
    mock_fee_accounts: [MockFeeAccountArgs; 2],
    MockProtocolFeeBps { trading, lp }: MockProtocolFeeBps,
) -> ProgramTest {
    let (mut program_test, mut pool_state) = lido_wsol_base_program_test(args);

    pool_state.pricing_program = flat_fee_interface::ID;
    pool_state.trading_protocol_fee_bps = trading;
    pool_state.lp_protocol_fee_bps = lp;

    program_test.add_account(
        flat_fee_lib::program::STATE_ID,
        MockProgramState(flat_fee_state).into_account(),
    );
    for mfa in mock_fee_accounts {
        let (acc, addr) = mfa.to_fee_account_and_addr(flat_fee_interface::ID);
        program_test.add_account(addr, MockFeeAccount(acc).into_account());
    }
    program_test.add_account(
        s_controller_lib::program::POOL_STATE_ID,
        MockPoolState(pool_state).into_account(),
    );

    program_test.add_flat_fee_program()
}

pub fn lido_wsol_base_program_test(
    LidoWsolProgramTestArgs {
        wsol_reserves,
        stsol_sol_value,
        stsol_reserves,
        wsol_protocol_fee_accumulator,
        stsol_protocol_fee_accumulator,
        lp_token_mint,
        lp_token_supply,
    }: LidoWsolProgramTestArgs,
) -> (ProgramTest, PoolState) {
    let mut program_test = ProgramTest::default();

    program_test = program_test
        .add_wsol_progs()
        .add_lido_progs()
        .add_lido_stake_pool()
        .add_mock_lst_states(&[
            MockLstStateArgs {
                mint: stsol::ID,
                sol_value: stsol_sol_value,
                reserves_amt: stsol_reserves,
                protocol_fee_accumulator_amt: stsol_protocol_fee_accumulator,
                token_program: spl_token::ID,
                sol_value_calculator: lido_calculator_lib::program::ID,
                is_input_disabled: false,
            },
            MockLstStateArgs {
                mint: native_mint::ID,
                sol_value: wsol_reserves,
                reserves_amt: wsol_reserves,
                protocol_fee_accumulator_amt: wsol_protocol_fee_accumulator,
                token_program: spl_token::ID,
                sol_value_calculator: wsol_calculator_lib::program::ID,
                is_input_disabled: false,
            },
        ])
        .add_mock_lp_mint(lp_token_mint, lp_token_supply);

    let total_sol_value = stsol_sol_value + wsol_reserves;

    let mut pool_state = DEFAULT_POOL_STATE;
    pool_state.total_sol_value = total_sol_value;
    pool_state.lp_token_mint = lp_token_mint;

    (program_test, pool_state)
}
