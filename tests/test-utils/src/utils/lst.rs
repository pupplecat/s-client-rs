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

pub fn try_find_disable_authority_index(
    pubkey_list: &Vec<Pubkey>,
    pubkey: Pubkey,
) -> Result<(usize, &Pubkey), SControllerError> {
    pubkey_list
        .iter()
        .enumerate()
        .find(|(_i, &s)| s == pubkey)
        .ok_or(SControllerError::InvalidDisablePoolAuthorityIndex)
}
