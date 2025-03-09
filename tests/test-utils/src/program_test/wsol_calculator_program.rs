use solana_program_test::{processor, ProgramTest};

pub trait WsolCalculatorProgramTest {
    fn add_wsol_progs(self) -> Self;
}

impl WsolCalculatorProgramTest for ProgramTest {
    fn add_wsol_progs(mut self) -> Self {
        self.prefer_bpf(false);
        self.add_program(
            "wsol_calculator",
            wsol_calculator_lib::program::ID,
            processor!(wsol_calculator::process_instruction),
        );

        self
    }
}
