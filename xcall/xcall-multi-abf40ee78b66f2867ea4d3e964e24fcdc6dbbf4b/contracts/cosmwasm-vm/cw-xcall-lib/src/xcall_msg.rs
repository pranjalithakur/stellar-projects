use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

use crate::{
    message::envelope::Envelope,
    network_address::{NetId, NetworkAddress},
};

#[cw_serde]
pub enum ExecuteMsg {
    SetAdmin {
        address: String,
    },
    SetProtocolFee {
        value: u128,
    },
    SetProtocolFeeHandler {
        address: String,
    },

    SendCallMessage {
        to: NetworkAddress,
        data: Vec<u8>,
        rollback: Option<Vec<u8>>,
        sources: Option<Vec<String>>,
        destinations: Option<Vec<String>>,
    },
    SendCall {
        envelope: Envelope,
        to: NetworkAddress,
    },
    HandleMessage {
        from_nid: NetId,
        msg: Vec<u8>,
    },

    HandleError {
        sn: u128,
    },
    ExecuteCall {
        request_id: u128,
        data: Vec<u8>,
    },

    ExecuteRollback {
        sequence_no: u128,
    },
    SetDefaultConnection {
        nid: NetId,
        address: Addr,
    },
}
