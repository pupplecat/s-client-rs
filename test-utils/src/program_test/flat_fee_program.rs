use solana_program_test::{processor, ProgramTest};

pub trait FlatFeeProgramTest {
    fn add_flat_fee_program(self) -> Self;
}

impl FlatFeeProgramTest for ProgramTest {
    fn add_flat_fee_program(mut self) -> Self {
        self.add_program(
            "flat_fee",
            flat_fee_lib::program::ID,
            processor!(flat_fee::entrypoint::process_instruction),
        );
        self
    }
}
