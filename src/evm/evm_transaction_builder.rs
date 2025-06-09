//! Transaction builder for EVM transactions
use crate::transaction_builder::TxBuilder;

use super::{
    evm_transaction::EVMTransaction,
    types::{AccessList, Address},
};

pub struct EVMTransactionBuilder {
    chain_id: Option<u64>,
    nonce: Option<u64>,
    to: Option<Address>,
    value: Option<u128>,
    input: Option<Vec<u8>>,
    gas_limit: Option<u128>,
    max_fee_per_gas: Option<u128>,
    max_priority_fee_per_gas: Option<u128>,
    access_list: Option<AccessList>,
}

impl Default for EVMTransactionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TxBuilder<EVMTransaction> for EVMTransactionBuilder {
    fn build(&self) -> EVMTransaction {
        EVMTransaction {
            chain_id: self.chain_id.expect("chain_id is mandatory"),
            nonce: self.nonce.expect("nonce is mandatory"),
            to: self.to,
            value: self.value.unwrap_or_default(),
            input: self.input.clone().unwrap_or_default(),
            gas_limit: self.gas_limit.expect("gas_limit is mandatory"),
            max_fee_per_gas: self.max_fee_per_gas.expect("max_fee_per_gas is mandatory"),
            max_priority_fee_per_gas: self.max_priority_fee_per_gas.unwrap_or_default(),
            access_list: self.access_list.clone().unwrap_or_default(),
        }
    }
}

impl EVMTransactionBuilder {
    pub const fn new() -> Self {
        Self {
            chain_id: None,
            nonce: None,
            to: None,
            value: None,
            input: None,
            gas_limit: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
            access_list: None,
        }
    }

    /// Chain ID of the transaction.
    pub const fn chain_id(mut self, chain_id: u64) -> Self {
        self.chain_id = Some(chain_id);
        self
    }

    /// Nonce of the transaction.
    pub const fn nonce(mut self, nonce: u64) -> Self {
        self.nonce = Some(nonce);
        self
    }

    /// Address of the recipient.
    pub const fn to(mut self, to: Address) -> Self {
        self.to = Some(to);
        self
    }

    /// Value attached to the transaction.
    pub const fn value(mut self, value: u128) -> Self {
        self.value = Some(value);
        self
    }

    /// Input data of the transaction.
    pub fn input(mut self, input: Vec<u8>) -> Self {
        self.input = Some(input);
        self
    }

    /// Gas limit of the transaction.
    pub const fn gas_limit(mut self, gas_limit: u128) -> Self {
        self.gas_limit = Some(gas_limit);
        self
    }

    /// Maximum fee per gas of the transaction.
    pub const fn max_fee_per_gas(mut self, max_fee_per_gas: u128) -> Self {
        self.max_fee_per_gas = Some(max_fee_per_gas);
        self
    }

    /// Maximum priority fee per gas of the transaction.
    pub const fn max_priority_fee_per_gas(mut self, max_priority_fee_per_gas: u128) -> Self {
        self.max_priority_fee_per_gas = Some(max_priority_fee_per_gas);
        self
    }

    /// Access list of the transaction.
    pub fn access_list(mut self, access_list: AccessList) -> Self {
        self.access_list = Some(access_list);
        self
    }
}

