use super::consts::{PRIMARY_VALIDATOR_PK, SECONDARY_VALIDATOR_PK};

pub struct BridgeEnvConfig {
    pub primary_validator_pk: String,
    pub secondary_validator_pk: String,

    /// default: `0`
    pub yaro_fee_share_bp: f64,
    /// default: `0`
    pub yusd_fee_share_bp: f64,

    pub yaro_admin_fee: u128,
    pub yusd_admin_fee: u128,

    /// default: `100_000.0`
    pub yaro_admin_deposit: f64,
    /// default: `100_000.0`
    pub yusd_admin_deposit: f64,
}

impl Default for BridgeEnvConfig {
    fn default() -> Self {
        BridgeEnvConfig {
            yaro_fee_share_bp: 0.0,
            yusd_fee_share_bp: 0.0,

            yaro_admin_fee: 0,
            yusd_admin_fee: 0,

            yaro_admin_deposit: 100_000.0,
            yusd_admin_deposit: 100_000.0,

            primary_validator_pk: PRIMARY_VALIDATOR_PK.to_owned(),
            secondary_validator_pk: SECONDARY_VALIDATOR_PK.to_owned(),
        }
    }
}
