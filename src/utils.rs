use axiom_codec::types::{field_elements::AnySubqueryResult, native::{Subquery, SubqueryResult}};
use axiom_eth::{
    mpt::KECCAK_RLP_EMPTY_STRING,
    providers::setup_provider,
    Field,
    halo2_base::AssignedValue,
    utils::component::{
        types::{FixLenLogical, Flatten},
        utils::get_logical_value,
         ComponentType, FlattenVirtualTable, LogicalResult,
    },
};
use axiom_query::{
    components::subqueries::{
        account::{types::{
        ComponentTypeAccountSubquery, FieldAccountSubqueryCall,
        }, STORAGE_ROOT_INDEX}, common::OutputSubqueryShard, storage::types::{CircuitInputStorageShard, CircuitInputStorageSubquery, ComponentTypeStorageSubquery
        }
        // common::{extract_logical_results, extract_virtual_table},
    },
    utils::codec::{AssignedAccountSubquery, AssignedStorageSubquery, AssignedStorageSubqueryResult},
};
use itertools::Itertools;
use rlp::RlpStream;
use crate::constants::*;
// use crate::types::CircuitInputStorageSubquery;
use anyhow::{Context as _, Result};
use axiom_eth::{
    halo2_base::Context, providers::storage::json_to_mpt_input,
    rlc::circuit::builder::RlcCircuitBuilder,
    storage::circuit::EthStorageInput, storage::EthStorageChip,
};
use ethers_core::types::{Address, Block, EIP1186ProofResponse, H160,H256,Chain};
use ethers_providers::{Middleware, Provider};
use tiny_keccak::{Hasher, Keccak};
// use crate::Field;

use zerocopy::AsBytes;

pub(crate) fn extract_virtual_table<
    F: Field,
    S: Into<Flatten<AssignedValue<F>>>,
    T: Into<Flatten<AssignedValue<F>>>,
>(
    outputs: impl Iterator<Item = AnySubqueryResult<S, T>>,
) -> FlattenVirtualTable<AssignedValue<F>> {
    outputs.map(|output| (output.subquery.into(), output.value.into())).collect()
}

pub(crate) fn extract_logical_results<
    F: Field,
    S: FixLenLogical<AssignedValue<F>>,
    FS: FixLenLogical<F>,
    T: ComponentType<F, InputValue = FS, InputWitness = S, LogicalInput = FS>,
>(
    outputs: impl Iterator<Item = AnySubqueryResult<S, T::OutputWitness>>,
) -> Vec<LogicalResult<F, T>> {
    outputs
        .map(|output| {
            LogicalResult::<F, T>::new(
                get_logical_value(&output.subquery),
                get_logical_value(&output.value),
            )
        })
        .collect()
}


// // /// https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/configs/production/all_max.yml#L91
// // pub const ACCOUNT_PROOF_MAX_DEPTH: usize = 14;
// // /// https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/configs/production/all_max.yml#L116
// // pub const STORAGE_PROOF_MAX_DEPTH: usize = 13;
// // /// The circuit will have 2^k rows.
// // const K: usize = 10;
// // /// If you need to use range checks, a good default is to set `lookup_bits` to 1 less than `k`.
// // const LOOKUP_BITS: usize = K - 1;
// // /// Constraints are ignored if set to true.
// // const WITNESS_GEN_ONLY: bool = false;
// // /// This means we can concatenate arrays with individual max length 2^32.
// // /// https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/src/lib.rs#L23
// // pub const DEFAULT_RLC_CACHE_BITS: usize = 32;
// // /// Storage slot of Safe's signedMessages mapping
// // pub const SAFE_SIGNED_MESSAGES_SLOT: [u8; 32] = [
// //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7,
// // ];
// // /// Index of the storage root in an account node.
// // pub const STORAGE_ROOT_INDEX: usize = 2;

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

pub fn json_to_input(
    block: Block<H256>,
    proof: EIP1186ProofResponse,
) -> EthStorageInput {
    let mut input = json_to_mpt_input(
        proof,
        ACCOUNT_PROOF_MAX_DEPTH,
        STORAGE_PROOF_MAX_DEPTH,
    );
    input.acct_pf.root_hash = block.state_root;
    input
}

pub async fn fetch_input(
    rpc: &str,
    safe_address: Address,
    msg_hash: H256,
    //      circuit_input, state_root, storage_root, address, block_number, header_rlp
) -> Result<(CircuitInputStorageSubquery, H256, H256, H256, H160, u32, Vec<u8>)> {
    let storage_key =
        keccak256(&concat_bytes64(msg_hash.into(), SAFE_SIGNED_MESSAGES_SLOT));

    let provider = Provider::try_from(rpc)?;
    let latest = provider.get_block_number().await?;
    let block = provider.get_block(latest).await?.context("no such block")?;
    let proof = provider
        .get_proof(safe_address, vec![storage_key.into()], Some(latest.into()))
        .await?;
    let storage_hash = if proof.storage_hash.is_zero() {
        // RPC provider may give zero storage hash for empty account, but the correct storage hash should be the null root = keccak256(0x80)
        H256::from_slice(&KECCAK_RLP_EMPTY_STRING)
    } else {
        proof.storage_hash
    };

    let block_number: u32 = block.number.unwrap().try_into().unwrap();
    let state_root = block.state_root;
    let header_rlp = rlp_encode_header(&block);
    Ok((CircuitInputStorageSubquery {
        block_number: block_number.into(),
        proof: json_to_input(block, proof),
    }, state_root, storage_hash.into(), H256::from(storage_key), safe_address.into(), block_number, header_rlp))
}

// pub fn rlc_builderz<F: Field>() -> (RlcCircuitBuilder<F>, RlcCircuitBuilder<F>)
// {
//     let mut builder1 =
//         RlcCircuitBuilder::new(WITNESS_GEN_ONLY, DEFAULT_RLC_CACHE_BITS)
//             .use_k(K);
//     builder1.set_lookup_bits(LOOKUP_BITS);
//     let mut builder2 =
//         RlcCircuitBuilder::new(WITNESS_GEN_ONLY, DEFAULT_RLC_CACHE_BITS)
//             .use_k(K);
//     builder2.set_lookup_bits(LOOKUP_BITS);
//     (builder1, builder2)
// }

pub fn to_address(addr: &str) -> Address {
    Address::from(
        const_hex::decode_to_array::<&str, 20>(addr).expect("address"),
    )
}

pub fn to_msg_hash(hash: &str) -> H256 {
    H256::from(const_hex::decode_to_array::<&str, 32>(hash).expect("msg hash"))
}

pub async fn test_fixture() -> Result<(CircuitInputStorageSubquery,H256,H256, H256,H160, u32, Vec<u8>)> {
    fetch_input("https://rpc.gnosis.gateway.fm", to_address("0x38Ba7f4278A1482FA0a7bC8B261a9A673336EDDc"), to_msg_hash("0xa225aed0c0283cef82b24485b8b28fb756fc9ce83d25e5cf799d0c8aa20ce6b7")).await
}

pub async fn get_latest_block_number(network: Chain) -> u64 {
    let provider = setup_provider(network);
    provider.get_block_number().await.unwrap().as_u64()
}

// subqery results preparation helpers
pub fn append(
    results: &mut Vec<SubqueryResult>,
    subqueries: &[(impl Into<Subquery> + Clone, H256)],
) {
    for (s, v) in subqueries {
        results.push(SubqueryResult { subquery: s.clone().into(), value: v.0.into() })
    }
}
pub fn resize_with_first<T: Clone>(v: &mut Vec<T>, cap: usize) {
    if cap > 0 {
        v.resize(cap, v[0].clone());
    } else {
        v.clear();
    }
}
pub fn prepare<A: Clone>(results: Vec<(A, H256)>) -> OutputSubqueryShard<A, H256> {
    let results = results.into_iter().map(|(s, v)| AnySubqueryResult::new(s, v)).collect_vec();
    OutputSubqueryShard { results }
}

// https://ethereum.stackexchange.com/a/67332
// https://github.com/ethereum/go-ethereum/blob/14eb8967be7acc54c5dc9a416151ac45c01251b6/core/types/block.go#L65
pub fn rlp_encode_header(block: &Block<H256>) -> Vec<u8> {
    let mut rlp = RlpStream::new();
    rlp.begin_list(20);
    rlp.append(&block.parent_hash);
    rlp.append(&block.uncles_hash);
    rlp.append(&block.author.expect("author"));
    rlp.append(&block.state_root);
    rlp.append(&block.transactions_root);
    rlp.append(&block.receipts_root);
    rlp.append(&block.logs_bloom.expect("logs_bloom"));
    rlp.append(&block.difficulty);
    rlp.append(&block.number.expect("number"));
    rlp.append(&block.gas_limit);
    rlp.append(&block.gas_used);
    rlp.append(&block.timestamp);
    rlp.append(&block.extra_data.as_bytes().to_vec());
    rlp.append(&block.mix_hash.expect("mix_hash"));
    rlp.append(&block.nonce.expect("nonce"));
    rlp.append(&block.base_fee_per_gas.expect("base_fee_per_gas")); // london
    rlp.append(&block.withdrawals_root.expect("withdrawals_root")); // shanghai
    rlp.append(&block.blob_gas_used.expect("blob_gas_used")); // cancun
    rlp.append(&block.excess_blob_gas.expect("excess_blob_gas")); // cancun
    rlp.append(
        &block
            .parent_beacon_block_root
            .expect("parent_beacon_block_root"),
    ); // cancun
    rlp.out().freeze().into()
}

/// Computes the Merkle Mountain Range root and proof for a single leaf.
pub fn mmr_1(leaf: H256) -> (H256, Vec<H256>) {
    let peak = keccak256(&concat_bytes64([0u8; 32], leaf.into()));
    let root = keccak256(&concat_bytes64(MMR_SIZE_1, peak)).into();
    let proof = vec![ZERO_32.into(), peak.into()];
    (root, proof)
}