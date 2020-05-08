use rlp_derive::{RlpEncodable, RlpDecodable};
use ethereum_types::{H256, U256};

#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct Account {
    pub nonce: U256,
    pub balance: U256,
    pub storage_root: H256,
    pub code_hash: H256,
}
