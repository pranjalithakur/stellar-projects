pub const PRIMARY_VALIDATOR_PK: &'static str =
    "b07d0e5f33e159bd7b471d0e79e4211205f4e89949247ec01ba7559b71acee77";
pub const SECONDARY_VALIDATOR_PK: &'static str =
    "cd3ae465374e1829e93c410a2ad1a5540a0351b49f567b94df9eefa66be76897";

/// System precision (digits)
pub const SP: u32 = 3;
/// System precision (exponent)
pub const ESP: u32 = 1_000;

pub const BP: u32 = 10000;

pub const GOERLI_CHAIN_ID: u32 = 2;
pub const OTHER_CHAIN_IDS: [u8; 32] = [
    0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

pub const GOERLI_GAS_PRICE: u128 = 200_000_000;
pub const GOERLI_PRICE: u128 = 1_000_000_000;

pub const THIS_GAS_PRICE: u128 = 400_000_000;
pub const THIS_PRICE: u128 = 20_000_000;
