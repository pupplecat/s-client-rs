use solana_program_test::{processor, ProgramTest};

pub trait SplCalculatorProgramTest {
    fn add_spl_calculator_program(self) -> Self;
}

impl SplCalculatorProgramTest for ProgramTest {
    fn add_spl_calculator_program(mut self) -> Self {
        self.prefer_bpf(false);
        self.add_program(
            "spl_calculator",
            spl_calculator_lib::program::ID,
            processor!(spl_calculator::entrypoint::process_instruction),
        );
        self
    }
}
