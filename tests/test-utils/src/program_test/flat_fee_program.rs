use solana_program_test::{processor, ProgramTest};

pub trait FlatFeeProgramTest {
    fn add_flat_fee_program(self) -> Self;

    fn add_no_fee_program(self) -> Self;
}

impl FlatFeeProgramTest for ProgramTest {
    fn add_flat_fee_program(mut self) -> Self {
        self.prefer_bpf(false);
        self.add_program(
            "flat_fee",
            flat_fee_lib::program::ID,
            processor!(flat_fee::entrypoint::process_instruction),
        );
        self
    }

    fn add_no_fee_program(mut self) -> Self {
        self.prefer_bpf(false);
        self.add_program(
            "no_fee_pricing_program",
            no_fee_pricing_program::ID,
            processor!(no_fee_pricing_program::process_instruction),
        );
        self
    }
}
