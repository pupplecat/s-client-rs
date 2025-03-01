use solana_program_test::{processor, ProgramTest};

pub trait LidoCalculatorProgramTest {
    fn add_lido_calculator_program(self) -> Self;
}

impl LidoCalculatorProgramTest for ProgramTest {
    fn add_lido_calculator_program(mut self) -> Self {
        self.prefer_bpf(false);
        self.add_program(
            "lido_calculator",
            lido_calculator_lib::program::ID,
            processor!(lido_calculator::entrypoint::process_instruction),
        );
        self
    }
}
