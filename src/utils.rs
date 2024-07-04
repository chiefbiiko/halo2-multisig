use crate::constants::*;
use anyhow::{Context as _, Result};
use axiom_codec::types::{
    field_elements::AnySubqueryResult,
    native::{Subquery, SubqueryResult},
};
use axiom_eth::{
    mpt::KECCAK_RLP_EMPTY_STRING,
    providers::storage::json_to_mpt_input,
    providers::{block::get_block_rlp, setup_provider},
    storage::circuit::EthStorageInput,
};
use axiom_query::components::subqueries::{block_header::MMR_MAX_NUM_PEAKS, common::OutputSubqueryShard};
use ethers_core::types::{Address, Block, Chain, EIP1186ProofResponse, H160, H256};
use ethers_providers::{Middleware, Provider};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tiny_keccak::{Hasher, Keccak};

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

/// Simple wrapper holding all component input data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Halo2MultisigInput {
    pub eth_storage_input: EthStorageInput,
    pub state_root: H256,
    pub storage_root: H256,
    pub storage_key: H256,
    pub address: H160,
    pub block_number: u32,
    pub block_hash: H256,
    pub header_rlp: Vec<u8>,
}

pub async fn fetch_input(rpc: &str, safe_address: Address, msg_hash: H256) -> Result<Halo2MultisigInput> {
    let storage_key = keccak256(&concat_bytes64(msg_hash.into(), SAFE_SIGNED_MESSAGES_SLOT));

    let provider = Provider::try_from(rpc)?;
    let latest = provider.get_block_number().await?;
    let block = provider.get_block(latest).await?.context("no such block")?;
    let proof = provider.get_proof(safe_address, vec![storage_key.into()], Some(latest.into())).await?;

    let storage_hash = if proof.storage_hash.is_zero() {
        // RPC provider may give zero storage hash for empty account, but the correct storage hash should be the null root = keccak256(0x80)
        H256::from_slice(&KECCAK_RLP_EMPTY_STRING)
    } else {
        proof.storage_hash
    };
    let block_number: u32 = block.number.unwrap().try_into().unwrap();
    let block_hash = block.hash.expect("block hash");
    let state_root = block.state_root;
    let header_rlp = get_block_rlp(&block);

    Ok(Halo2MultisigInput {
        eth_storage_input: json_to_input(block, proof),
        state_root,
        storage_root: storage_hash.into(),
        storage_key: H256::from(storage_key),
        address: safe_address,
        block_number,
        block_hash,
        header_rlp,
    })
}

pub fn to_address(addr: &str) -> Address {
    Address::from(const_hex::decode_to_array::<&str, 20>(addr).expect("address"))
}

pub fn to_msg_hash(hash: &str) -> H256 {
    H256::from(const_hex::decode_to_array::<&str, 32>(hash).expect("msg hash"))
}

pub async fn test_input() -> Result<Halo2MultisigInput> {
    fetch_input(
        "https://rpc.gnosis.gateway.fm",
        to_address("0x38Ba7f4278A1482FA0a7bC8B261a9A673336EDDc"),
        to_msg_hash("0xa225aed0c0283cef82b24485b8b28fb756fc9ce83d25e5cf799d0c8aa20ce6b7"),
    )
    .await
}

pub async fn get_latest_block_number(network: Chain) -> u64 {
    let provider = setup_provider(network);
    provider.get_block_number().await.unwrap().as_u64()
}

// subqery results preparation helpers
pub fn append(results: &mut Vec<SubqueryResult>, subqueries: &[(impl Into<Subquery> + Clone, H256)]) {
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

// /// Computes the Merkle Mountain Range root, peak, and proof for a single leaf.
// pub fn mmr_1(leaf: &H256) -> (H256, H256, Vec<H256>) {
//     let peak = keccak256(&concat_bytes64(ZERO_32, (*leaf).into()));
//     let root = keccak256(&concat_bytes64(MMR_SIZE_1, peak)).into();
//     let proof = vec![ZERO_32.into(), *leaf];
//     (root, peak.into(), proof)
// }

//FROM https://github.com/axiom-crypto/axiom-v2-periphery/blob/6482de3f73618b0df11f0955af2e1d1fb5a67a8e/src/libraries/MerkleTree.sol#L15
pub const HISTORICAL_NUM_ROOTS: usize = 1024;
pub fn merkle_root(leaves: [H256; HISTORICAL_NUM_ROOTS]) -> [u8; 32] {
    // we create a new array to avoid mutating `leaves`, which is passed by reference
    // unnecessary if calldata `leaves` is passed in since it is automatically copied to memory
    let mut hashes: [H256; HISTORICAL_NUM_ROOTS / 2] = [H256::zero(); HISTORICAL_NUM_ROOTS / 2];
    for i in 0..(HISTORICAL_NUM_ROOTS / 2) {
        hashes[i] = keccak256(concat_bytes64(leaves[i << 1].into(), leaves[(i << 1) | 1].into())).into();
    }
    let mut len = HISTORICAL_NUM_ROOTS / 4;
    while len != 0 {
        for i in 0..len {
            hashes[i] = keccak256(concat_bytes64(hashes[i << 1].into(), hashes[(i << 1) | 1].into())).into();
        }
        len = len / 2;
    }
    return hashes[0].into();
}

// /// Computes the Merkle Mountain Range peaks, and proof for a single leaf.
// /// A leaf is a Merkle tree root of 1024 consecutive block hashes.
// /// https://github.com/axiom-crypto/axiom-docs/blob/main/docs/protocol/protocol-design/caching-block-hashes.md
// pub fn mmr_1(leaf: &H256) -> (/*H256,*/ [H256; MMR_MAX_NUM_PEAKS], [H256; MMR_MAX_NUM_PEAKS - 1]) {
//     // build merkle tree with 1024 leafs and only the first being the blockhash while the rest are all zeros
//     let mut leaves = [H256::zero(); HISTORICAL_NUM_ROOTS];
//     leaves[0] = *leaf;

//     let mroot1024 = merkle_root(leaves);

//     // let peak = keccak256(&concat_bytes64(ZERO_32, mroot1024));
//     let peak = keccak256(&mroot1024);

//     // let root = keccak256(&concat_bytes64(MMR_SIZE_1, peak)).into();
    
//     let mut mmr_proof = [H256::zero(); MMR_MAX_NUM_PEAKS - 1];
//     // mmr_proof[0] = ZERO_32.into();
//     // mmr_proof[1] = mroot1024.into();
//     mmr_proof[0] = mroot1024.into();

//     let mut mmr_peaks = [H256::zero(); MMR_MAX_NUM_PEAKS];
//     mmr_peaks[0] = peak.into();

//     (/*root,*/ mmr_peaks, mmr_proof)
// }

///////////////////////////////////////////////////////////////////////////////

/// Computes the Merkle Mountain Range peaks, and proof for a single leaf.
/// A leaf is a Merkle tree root of 1024 consecutive block hashes.
/// https://github.com/axiom-crypto/axiom-docs/blob/main/docs/protocol/protocol-design/caching-block-hashes.md
pub fn mmr_1(leaf: &H256) -> (/*H256,*/ [H256; MMR_MAX_NUM_PEAKS], [H256; MMR_MAX_NUM_PEAKS - 1]) {
    // build merkle tree with 1024 leafs and only the first being the blockhash while the rest are all zeros
    let mut leaves = [H256::zero(); HISTORICAL_NUM_ROOTS];
    leaves[0] = *leaf;

    let mroot1024 = merkle_root(leaves);

    let peaks = [H256::zero(); MAX_MMR_PEAKS];
    let mut peaks_len = 0;

    //begin appendLeaf
    let new_peak = *leaf;
    let i = 0;
    let peaks_len = 0;
    while i < peaks_len && peaks[i] != H256::zero() {
        new_peak = keccak256(concat_bytes64(peaks[i].into(), new_peak.into())).into();
        peaks[i] = H256::zero();
        i = i + 1;
    }
    peaks[i] = new_peak;

    //shouldn't be the case for us
    // if (i >= peaksLength) {
    //     self.peaksLength = i + 1;
    // }
    //we dont need it as long as we only append 1 leaf
    // peaksChanged = i + 1;
    //end appendLeaf




    


////////////

    // let peak = keccak256(&concat_bytes64(ZERO_32, mroot1024));
    // let peak = keccak256(&mroot1024);

    //WIP workout pmmr commitment
    // return keccak256(abi.encodePacked(self.paddedLeaf, peaks));
    // let padded_leaf = mroot1024;
    // // peaks[i] = root(list[((len >> i) << i) - 2^i : ((len >> i) << i)])` if 2^i & len != 0, otherwise 0
    // let peaks = [H256::zero(); MAX_MMR_PEAKS];
    // let mut peaks_len = 0;
    
    //TODO mmr append leaf:=


    //TODO pmmr commit() 

    /////////
    // /// @notice Append a new element to the underlying list of the MMR
    // /// @param  self The MMR
    // /// @param  leaf The new element to append
    // /// @return peaksChanged self.peaks[0 : peaksChanged] have been changed
    // function appendLeaf(MMR memory self, bytes32 leaf) internal pure returns (uint256 peaksChanged) {
    //     unchecked {
    //         bytes32 newPeak = leaf;
    //         uint256 i;
    //         uint256 peaksLength = self.peaksLength;
    //         for (; i < peaksLength && self.peaks[i] != bytes32(0);) {
    //             newPeak = Hash.keccak(self.peaks[i], newPeak);
    //             delete self.peaks[i];
    //             ++i;
    //         }
    //         self.peaks[i] = newPeak;

    //         if (i >= peaksLength) {
    //             self.peaksLength = i + 1;
    //         }

    //         peaksChanged = i + 1;
    //     }
    // }




    /////////

    // let root = keccak256(&concat_bytes64(MMR_SIZE_1, peak)).into();
    
    // let mut mmr_proof = [H256::zero(); MMR_MAX_NUM_PEAKS - 1];
    // // mmr_proof[0] = ZERO_32.into();
    // // mmr_proof[1] = mroot1024.into();
    // mmr_proof[0] = mroot1024.into();

    // let mut mmr_peaks = [H256::zero(); MMR_MAX_NUM_PEAKS];
    // mmr_peaks[0] = peak.into();

    // (/*root,*/ mmr_peaks, mmr_proof)


}
