use ethers::prelude::*;
use ethers::types::transaction::eip712::Eip712;
use ethers_derive_eip712::*;

#[derive(Debug, Clone, Eip712, EthAbiType)]
#[eip712(
    name = "Postcard",
    version = "1",
    chain_id = 1,
    verifying_contract = "0x0000000000000000000000000000000000000001",
    salt = "and-pepper"
)]
pub(crate) struct PostcardClaim {
    pub foo: String,
}

fn main() {
    println!("Hello, world!");
}
