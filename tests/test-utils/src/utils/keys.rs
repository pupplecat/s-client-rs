pub mod jitosol {
    use std::str::FromStr;

    use solana_sdk::pubkey::Pubkey;

    pub fn id() -> Pubkey {
        Pubkey::from_str("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn").unwrap()
    }
}

pub mod jito_stake_pool {
    use std::str::FromStr;

    use solana_sdk::pubkey::Pubkey;

    pub fn id() -> Pubkey {
        Pubkey::from_str("Jito4APyf642JPZPx3hGc6WWJ8zPKtRbRs4P815Awbb").unwrap()
    }
}
