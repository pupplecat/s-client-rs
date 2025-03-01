use solana_banks_interface::{BanksTransactionResultWithSimulation, TransactionStatus};
use solana_program::program_pack::Pack;
use solana_program_test::{
    BanksClientError, ProgramTest, ProgramTestBanksClientExt, ProgramTestContext,
};
use solana_sdk::{
    clock::Clock,
    compute_budget,
    genesis_config::GenesisConfig,
    instruction::Instruction,
    native_token::LAMPORTS_PER_SOL,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    system_instruction,
    transaction::Transaction,
};

pub struct ProgramSimulator {
    program_test_context: ProgramTestContext,
}

impl ProgramSimulator {
    pub async fn start_from_program_test(program_test: ProgramTest) -> Self {
        let program_test_context = program_test.start_with_context().await;

        Self {
            program_test_context,
        }
    }

    pub async fn process_ix_with_default_compute_limit(
        &mut self,
        instruction: Instruction,
        signers: &Vec<&Keypair>,
        payer: Option<&Keypair>,
    ) -> Result<Signature, BanksClientError> {
        self.process_ixs_with_default_compute_limit(&[instruction], signers, payer)
            .await
    }

    pub async fn process_ixs_with_default_compute_limit(
        &mut self,
        instructions: &[Instruction],
        signers: &Vec<&Keypair>,
        payer: Option<&Keypair>,
    ) -> Result<Signature, BanksClientError> {
        // Create the compute budget instruction
        let compute_units_ix =
            compute_budget::ComputeBudgetInstruction::set_compute_unit_limit(2_000_000);

        // Add the compute budget instruction to the provided instructions
        let mut all_instructions = Vec::with_capacity(instructions.len() + 1);
        all_instructions.push(compute_units_ix);
        all_instructions.extend_from_slice(instructions);

        // Determine the actual payer (default to genesis keypair if not provided)
        let actual_payer = payer.unwrap_or(&self.program_test_context.payer);

        // Create the transaction
        let mut transaction =
            Transaction::new_with_payer(&all_instructions, Some(&actual_payer.pubkey()));

        // Get a new blockhash
        let blockhash = self
            .program_test_context
            .banks_client
            .get_new_latest_blockhash(&self.program_test_context.last_blockhash)
            .await
            .unwrap();
        self.program_test_context.last_blockhash = blockhash;

        // Partially sign the transaction with the payer and additional signers
        transaction.partial_sign(&[actual_payer], self.program_test_context.last_blockhash);
        transaction.partial_sign(signers, self.program_test_context.last_blockhash);

        let signature = transaction.signatures[0];

        // Process the transaction
        self.program_test_context
            .banks_client
            .process_transaction(transaction)
            .await?;

        Ok(signature)
    }

    pub async fn simulate_ix_with_default_compute_limit(
        &mut self,
        instruction: Instruction,
        signers: &Vec<&Keypair>,
        payer: Option<&Keypair>,
    ) -> Result<BanksTransactionResultWithSimulation, BanksClientError> {
        self.simulate_ixs_with_default_compute_limit(&[instruction], signers, payer)
            .await
    }

    pub async fn simulate_ixs_with_default_compute_limit(
        &mut self,
        instructions: &[Instruction],
        signers: &Vec<&Keypair>,
        payer: Option<&Keypair>,
    ) -> Result<BanksTransactionResultWithSimulation, BanksClientError> {
        // Create the compute budget instruction
        let compute_units_ix =
            compute_budget::ComputeBudgetInstruction::set_compute_unit_limit(2_000_000);

        // Add the compute budget instruction to the provided instructions
        let mut all_instructions = Vec::with_capacity(instructions.len() + 1);
        all_instructions.push(compute_units_ix);
        all_instructions.extend_from_slice(instructions);

        // Determine the actual payer (default to genesis keypair if not provided)
        let actual_payer = payer.unwrap_or(&self.program_test_context.payer);

        // Create the transaction
        let mut transaction =
            Transaction::new_with_payer(&all_instructions, Some(&actual_payer.pubkey()));

        // Get a new blockhash
        let blockhash = self
            .program_test_context
            .banks_client
            .get_new_latest_blockhash(&self.program_test_context.last_blockhash)
            .await
            .unwrap();
        self.program_test_context.last_blockhash = blockhash;

        // Partially sign the transaction with the payer and additional signers
        transaction.partial_sign(&[actual_payer], self.program_test_context.last_blockhash);
        transaction.partial_sign(signers, self.program_test_context.last_blockhash);

        // Process the transaction
        self.program_test_context
            .banks_client
            .simulate_transaction(transaction)
            .await
    }

    pub async fn airdrop(
        &mut self,
        to: &Pubkey,
        lamports: u64,
    ) -> Result<Signature, BanksClientError> {
        let instruction =
            system_instruction::transfer(&self.program_test_context.payer.pubkey(), to, lamports);

        self.process_ix_with_default_compute_limit(instruction, &vec![], None)
            .await
    }

    pub async fn get_funded_keypair(&mut self) -> Result<Keypair, BanksClientError> {
        let keypair = Keypair::new();
        self.airdrop(&keypair.pubkey(), LAMPORTS_PER_SOL).await?;
        Ok(keypair)
    }

    pub async fn get_packed_account_data<T: Pack + IsInitialized>(
        &mut self,
        pubkey: Pubkey,
    ) -> Result<T, BanksClientError> {
        let account = self
            .program_test_context
            .banks_client
            .get_account(pubkey)
            .await
            .map_err(|_err| BanksClientError::ClientError("Error fetching account"))?
            .ok_or_else(|| BanksClientError::ClientError("Account not found"))?;

        T::unpack(&account.data[..]).map_err(|_err| BanksClientError::ClientError("Unpack error"))
    }

    pub async fn get_balance(&mut self, pubkey: Pubkey) -> Result<u64, BanksClientError> {
        let lamports = self
            .program_test_context
            .banks_client
            .get_balance(pubkey)
            .await?;
        Ok(lamports)
    }

    pub async fn get_clock(&mut self) -> Result<Clock, BanksClientError> {
        self.program_test_context
            .banks_client
            .get_sysvar::<Clock>()
            .await
    }

    pub fn get_genesis_config(&mut self) -> Result<GenesisConfig, BanksClientError> {
        let config = self.program_test_context.genesis_config();

        Ok(config.clone())
    }

    pub async fn advance_clock_by(
        &mut self,
        seconds_to_advance: i64,
    ) -> Result<(), BanksClientError> {
        let mut clock = self
            .program_test_context
            .banks_client
            .get_sysvar::<Clock>()
            .await?;

        clock.epoch_start_timestamp += seconds_to_advance;
        clock.unix_timestamp += seconds_to_advance;
        self.program_test_context.set_sysvar(&clock);

        Ok(())
    }

    pub async fn advance_clock_to(
        &mut self,
        target_timestamp: i64,
    ) -> Result<(), BanksClientError> {
        let mut clock = self
            .program_test_context
            .banks_client
            .get_sysvar::<Clock>()
            .await?;

        clock.epoch_start_timestamp = target_timestamp;
        clock.unix_timestamp = target_timestamp;
        self.program_test_context.set_sysvar(&clock);

        Ok(())
    }

    pub async fn get_transaction_status(
        &mut self,
        signature: Signature,
    ) -> Result<Option<TransactionStatus>, BanksClientError> {
        self.program_test_context
            .banks_client
            .get_transaction_status(signature)
            .await
    }

    pub fn warp_to_epoch(&mut self, warp_epoch: u64) -> Result<(), BanksClientError> {
        self.program_test_context.warp_to_epoch(warp_epoch).unwrap();

        Ok(())
    }

    pub fn warp_to_slot(&mut self, warp_slot: u64) -> Result<(), BanksClientError> {
        self.program_test_context.warp_to_slot(warp_slot).unwrap();

        Ok(())
    }
}
