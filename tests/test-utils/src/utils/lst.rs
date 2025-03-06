use s_controller_interface::{LstState, SControllerError};
use solana_sdk::pubkey::Pubkey;

pub fn try_find_lst_state_index(
    lst_state_list: &Vec<LstState>,
    lst_mint: Pubkey,
) -> Result<(usize, &LstState), SControllerError> {
    lst_state_list
        .iter()
        .enumerate()
        .find(|(_i, s)| s.mint == lst_mint)
        .ok_or(SControllerError::InvalidLstIndex)
}
