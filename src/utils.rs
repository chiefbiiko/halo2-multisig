use axiom_eth::{storage::circuit::EthStorageInput,providers::storage::json_to_mpt_input,};
use anyhow::{Context, Result};
use ethers_core::types::{Address,EIP1186ProofResponse, Block, H256};
use ethers_providers::{Middleware, Provider};
use tiny_keccak::{Hasher, Keccak};
use crate::types::CircuitInputStorageSubquery;

/// https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/configs/production/all_max.yml#L91
pub const ACCOUNT_PROOF_MAX_DEPTH: usize = 14;
/// https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/configs/production/all_max.yml#L116
pub const STORAGE_PROOF_MAX_DEPTH: usize = 13;

pub const SAFE_SIGNED_MESSAGES_SLOT: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7,
];

pub fn concat_bytes64(a: [u8; 32], b: [u8; 32]) -> [u8; 64] {
    // https://stackoverflow.com/a/76573243
    unsafe { core::mem::transmute::<[[u8; 32]; 2], [u8; 64]>([a, b]) }
}

pub fn keccak256<T: AsRef<[u8]>>(input: T) -> [u8; 32] {
    let mut out = [0u8; 32];
    let mut k = Keccak::v256();
    k.update(input.as_ref());
    k.finalize(&mut out);
    out
}

pub fn json_to_input(block: Block<H256>, proof: EIP1186ProofResponse) -> EthStorageInput {
    let mut input = json_to_mpt_input(proof, ACCOUNT_PROOF_MAX_DEPTH, STORAGE_PROOF_MAX_DEPTH);
    input.acct_pf.root_hash = block.state_root;
    input
}

pub async fn fetch_input(rpc: &str, safe_address: Address, msg_hash: H256) -> Result<CircuitInputStorageSubquery> {
    let storage_key = keccak256(&concat_bytes64(msg_hash.into(), SAFE_SIGNED_MESSAGES_SLOT));

    let provider = Provider::try_from(rpc)?;
    let latest = provider.get_block_number().await?;
    let block = provider.get_block(latest).await?.context("no such block")?;
    let proof = provider
        .get_proof(safe_address, vec![storage_key.into()], Some(latest.into()))
        .await?;

    Ok( CircuitInputStorageSubquery {
        block_number: block.number.unwrap().as_u64(),
        proof: json_to_input(block, proof)
    })
}

#[cfg(test)]
pub fn to_address(addr: &str) -> Address {
    Address::from(const_hex::decode_to_array::<&str, 20>(addr).expect("address"))
}

#[cfg(test)]
pub fn to_msg_hash(hash: &str) -> H256 {
    H256::from(const_hex::decode_to_array::<&str, 32>(hash).expect("msg hash"))
}

#[cfg(test)]
pub async fn test_fixture() -> Result<CircuitInputStorageSubquery> {
    fetch_input("https://rpc.gnosis.gateway.fm", to_address("0x38Ba7f4278A1482FA0a7bC8B261a9A673336EDDc"), to_msg_hash("0xa225aed0c0283cef82b24485b8b28fb756fc9ce83d25e5cf799d0c8aa20ce6b7")).await
}