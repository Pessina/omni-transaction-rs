use serde::{Deserialize, Serialize};
use rlp::RlpStream;
use schemars::JsonSchema;

use crate::constants::EIP_1559_TYPE;

use super::types::{AccessList, Address, Signature};
use super::utils::parse_eth_address;

///
/// ###### Example:
///
/// ```rust
/// let nonce: u64 = 0;
/// let to: Address = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
/// let value = 10000000000000000u128; // 0.01 ETH
/// let data: Vec<u8> = vec![];
/// let chain_id = 1;
/// let to_address_str = "d8dA6BF26964aF9D7eEd9e03E53415D37aA96045";
/// let to_address = Some(parse_eth_address(to_address_str));
/// // Generate using EVMTransaction
/// let tx = EVMTransaction {
///     chain_id,
///     nonce,
///     to: to_address,
///     value,
///     input: data.clone(),
///     gas_limit: GAS_LIMIT,
///     max_fee_per_gas: MAX_FEE_PER_GAS,
///     max_priority_fee_per_gas: MAX_PRIORITY_FEE_PER_GAS,
///     access_list: vec![],
/// };
/// ```
///
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EVMTransaction {
    pub chain_id: u64,
    pub nonce: u64,
    pub to: Option<Address>,
    pub value: u128,
    pub input: Vec<u8>,
    pub gas_limit: u128,
    pub max_fee_per_gas: u128,
    pub max_priority_fee_per_gas: u128,
    pub access_list: AccessList,
}

impl EVMTransaction {
    pub fn build_for_signing(&self) -> Vec<u8> {
        let mut rlp_stream = RlpStream::new();

        rlp_stream.append(&EIP_1559_TYPE);

        rlp_stream.begin_unbounded_list();

        self.encode_fields(&mut rlp_stream);

        rlp_stream.finalize_unbounded_list();

        rlp_stream.out().to_vec()
    }

    pub fn build_with_signature(&self, signature: &Signature) -> Vec<u8> {
        let mut rlp_stream = RlpStream::new();

        rlp_stream.append(&EIP_1559_TYPE);

        rlp_stream.begin_unbounded_list();

        self.encode_fields(&mut rlp_stream);

        rlp_stream.append(&signature.v);
        rlp_stream.append(&signature.r);
        rlp_stream.append(&signature.s);

        rlp_stream.finalize_unbounded_list();

        rlp_stream.out().to_vec()
    }

    fn encode_fields(&self, rlp_stream: &mut RlpStream) {
        let to: Vec<u8> = self.to.map_or(vec![], |to| to.to_vec());
        let access_list = self.access_list.clone();

        rlp_stream.append(&self.chain_id);
        rlp_stream.append(&self.nonce);
        rlp_stream.append(&self.max_priority_fee_per_gas);
        rlp_stream.append(&self.max_fee_per_gas);
        rlp_stream.append(&self.gas_limit);
        rlp_stream.append(&to);
        rlp_stream.append(&self.value);
        rlp_stream.append(&self.input);

        // Write access list.
        {
            rlp_stream.begin_unbounded_list();
            for access in access_list {
                rlp_stream.begin_unbounded_list();
                rlp_stream.append(&access.0.to_vec());
                // Append list of storage keys.
                {
                    rlp_stream.begin_unbounded_list();
                    for storage_key in access.1 {
                        rlp_stream.append(&storage_key.to_vec());
                    }
                    rlp_stream.finalize_unbounded_list();
                }
                rlp_stream.finalize_unbounded_list();
            }
            rlp_stream.finalize_unbounded_list();
        }
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        let v: serde_json::Value = serde_json::from_str(json)?;

        let to = v["to"].as_str().unwrap_or_default().to_string();

        let to_parsed = parse_eth_address(
            to.strip_prefix("0x")
                .unwrap_or("0000000000000000000000000000000000000000"),
        );

        let nonce_str = v["nonce"].as_str().expect("nonce should be provided");
        let nonce = parse_u64(nonce_str).expect("nonce should be a valid u64");

        let value_str = v["value"].as_str().expect("value should be provided");
        let value = parse_u128(value_str).expect("value should be a valid u128");

        let gas_limit_str = v["gasLimit"].as_str().expect("gasLimit should be provided");
        let gas_limit = parse_u128(gas_limit_str).expect("gasLimit should be a valid u128");

        let max_priority_fee_per_gas_str = v["maxPriorityFeePerGas"]
            .as_str()
            .expect("maxPriorityFeePerGas should be provided");
        let max_priority_fee_per_gas = parse_u128(max_priority_fee_per_gas_str)
            .expect("maxPriorityFeePerGas should be a valid u128");

        let max_fee_per_gas_str = v["maxFeePerGas"]
            .as_str()
            .expect("maxFeePerGas should be provided");
        let max_fee_per_gas =
            parse_u128(max_fee_per_gas_str).expect("maxFeePerGas should be a valid u128");

        let chain_id_str = v["chainId"].as_str().expect("chainId should be provided");
        let chain_id = parse_u64(chain_id_str).expect("chainId should be a valid u64");

        let input = v["input"].as_str().unwrap_or_default().to_string();
        let input =
            hex::decode(input.strip_prefix("0x").unwrap_or("")).expect("input should be hex");

        // TODO: Implement access list
        // let access_list = v["accessList"].as_str().unwrap_or_default().to_string();

        Ok(Self {
            chain_id,
            nonce,
            to: Some(to_parsed),
            value,
            input,
            gas_limit,
            max_fee_per_gas,
            max_priority_fee_per_gas,
            access_list: vec![],
        })
    }
}

fn parse_u64(value: &str) -> Result<u64, std::num::ParseIntError> {
    value.strip_prefix("0x").map_or_else(
        || value.parse::<u64>(),
        |hex_str| u64::from_str_radix(hex_str, 16),
    )
}

fn parse_u128(value: &str) -> Result<u128, std::num::ParseIntError> {
    value.strip_prefix("0x").map_or_else(
        || value.parse::<u128>(),
        |hex_str| u128::from_str_radix(hex_str, 16),
    )
}
