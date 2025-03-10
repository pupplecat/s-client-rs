use solana_sdk::{instruction::Instruction, program_error::ProgramError, pubkey::Pubkey};
use spl_token::instruction::transfer_checked;

#[derive(Clone, Copy, Debug)]
pub struct TransferCheckedKeys {
    pub token_program: Pubkey,
    pub from: Pubkey,
    pub mint: Pubkey,
    pub to: Pubkey,
    pub authority: Pubkey,
}

#[derive(Clone, Copy, Debug)]
pub struct TransferCheckedArgs {
    pub amount: u64,
    pub decimals: u8,
}

pub fn transfer_checked_ix(
    TransferCheckedKeys {
        token_program,
        from,
        mint,
        to,
        authority,
    }: TransferCheckedKeys,
    TransferCheckedArgs { amount, decimals }: TransferCheckedArgs,
) -> Result<Instruction, ProgramError> {
    transfer_checked(
        &token_program,
        &from,
        &mint,
        &to,
        &authority,
        &[],
        amount,
        decimals,
    )
}
