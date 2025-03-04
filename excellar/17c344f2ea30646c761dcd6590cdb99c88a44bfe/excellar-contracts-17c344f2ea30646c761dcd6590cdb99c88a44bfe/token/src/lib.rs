#![no_std]

extern crate alloc;

pub use crate::contract::ExcellarTokenClient;

mod admin;
mod allowance;
mod amm;
mod balance;
mod contract;
mod event;
mod metadata;
mod reward;
mod storage_types;
mod test;
