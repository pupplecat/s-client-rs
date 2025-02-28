use solana_program_test::ProgramTest;

use crate::program_test::{
    FlatFeeProgramTest, LidoCalculatorProgramTest, MarinadeCalculatorProgramTest,
    SControllerProgramTest, SplCalculatorProgramTest,
};
use cargo_metadata::MetadataCommand;

pub struct ProgramTestFixtures {}

fn set_sbf_out_dir() {
    // Use cargo_metadata to fetch workspace metadata
    let metadata = MetadataCommand::new()
        .exec()
        .expect("Failed to fetch workspace metadata");

    let workspace_root = metadata.workspace_root;

    // Construct and set the SBF output directory
    let sbf_out_dir = workspace_root.join("test-fixtures");

    dbg!(&sbf_out_dir);

    std::env::set_var("SBF_OUT_DIR", sbf_out_dir);
}

impl ProgramTestFixtures {
    pub fn setup() -> Self {
        set_sbf_out_dir();

        let program_test: ProgramTest = ProgramTest::default()
            .add_s_controller_program()
            .add_flat_fee_program()
            .add_lido_calculator_program()
            .add_marinade_calculator_program()
            .add_spl_calculator_program();

        Self {}
    }
}
