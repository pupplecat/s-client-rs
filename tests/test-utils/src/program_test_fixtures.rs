use program_simulator::ProgramSimulator;
use solana_program_test::{BanksClientError, ProgramTest};
use solana_sdk::{
    program_pack::Pack, pubkey::Pubkey, rent::Rent, signature::Keypair, signer::Signer,
    system_instruction::create_account,
};
use spl_token::instruction::initialize_mint;

use crate::program_test::{
    FlatFeeProgramTest, LidoCalculatorProgramTest, MarinadeCalculatorProgramTest,
    SControllerProgramTest, SplCalculatorProgramTest,
};
use cargo_metadata::MetadataCommand;

pub struct ProgramTestFixtures {
    pub program_simulator: ProgramSimulator,
    pub payer: Keypair,
}

pub fn set_sbf_out_dir() {
    // Use cargo_metadata to fetch workspace metadata
    let metadata = MetadataCommand::new()
        .exec()
        .expect("Failed to fetch workspace metadata");

    let workspace_root = metadata.workspace_root;

    // Construct and set the SBF output directory
    let sbf_out_dir = workspace_root.join("test-fixtures");

    // dbg!(&sbf_out_dir);

    std::env::set_var("SBF_OUT_DIR", sbf_out_dir);
}

impl ProgramTestFixtures {
    pub async fn setup_test_fixtures() -> Self {
        set_sbf_out_dir();

        let program_test: ProgramTest = ProgramTest::default()
            .add_flat_fee_program()
            .add_no_fee_program()
            .add_lido_progs()
            .add_lido_stake_pool()
            .add_marinade_progs()
            .add_marinade_stake_pool()
            .add_spl_progs()
            .add_jito_stake_pool()
            .add_s_controller_program();

        ProgramTestFixtures::setup(program_test).await
    }

    pub async fn setup(program_test: ProgramTest) -> Self {
        let mut program_simulator = ProgramSimulator::start_from_program_test(program_test).await;

        let payer = program_simulator.get_funded_keypair().await.unwrap();

        Self {
            program_simulator,
            payer,
        }
    }

    pub async fn create_mint(
        &mut self,
        initial_authority_pubkey: &Pubkey,
        decimals: u8,
    ) -> Result<Pubkey, BanksClientError> {
        let mint_keypair = Keypair::new();

        let create_account_instruction = create_account(
            &self.payer.pubkey(),
            &mint_keypair.pubkey(),
            Rent::default().minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN as u64,
            &spl_token::ID,
        );

        let initialize_mint_instruction = initialize_mint(
            &spl_token::ID,
            &mint_keypair.pubkey(),
            initial_authority_pubkey,
            Some(initial_authority_pubkey),
            decimals,
        )
        .unwrap();

        self.program_simulator
            .process_ixs_with_default_compute_limit(
                &[create_account_instruction, initialize_mint_instruction],
                &vec![&mint_keypair],
                Some(&self.payer),
            )
            .await?;

        Ok(mint_keypair.pubkey())
    }
}
