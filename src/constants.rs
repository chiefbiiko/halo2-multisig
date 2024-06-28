//FROM https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/configs/production/all_max.yml#L91
pub const ACCOUNT_PROOF_MAX_DEPTH: usize = 14;
//FROM https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/configs/production/all_max.yml#L116
pub const STORAGE_PROOF_MAX_DEPTH: usize = 13;
/// The circuit will have 2^k rows.
/// Our header shard gate chip has 63_710_878 advice cells in phase 0
/// With 19 advice columns that's 3_315_789.4736842103 rows 
/// 2**21=2_097_152; 2**22=4_194_304
pub const K: usize = 22;
/// If you need to use range checks, a good default is to set `lookup_bits` to 1 less than `k`.
pub const LOOKUP_BITS: usize = K - 1;
/// Constraints are ignored if set to true.
pub const WITNESS_GEN_ONLY: bool = false;
/// This means we can concatenate arrays with individual max length 2^32.
//FROM https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/src/lib.rs#L23
pub const DEFAULT_RLC_CACHE_BITS: usize = 32;
/// Storage slot of Safe's signedMessages mapping
pub const SAFE_SIGNED_MESSAGES_SLOT: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 7,
];
/// Index of the storage root in an account node.
pub const STORAGE_ROOT_INDEX: usize = 2;
/// Index of the state root in a block header.
pub const STATE_ROOT_INDEX: usize = 3;
//FROM https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-eth/configs/tests/storage.json#L10
pub const NUM_RLC_COLUMNS:usize = 3;
//FROM https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/configs/test/subquery_aggregation_for_agg.json#L4
pub const NUM_ADVICE: usize = 19;
//FROM https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/configs/test/subquery_aggregation_for_agg.json#L5
pub const NUM_LOOKUP_ADVICE: usize = 3;
//FROM https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/configs/test/subquery_aggregation_for_agg.json#L6
pub const NUM_FIXED: usize = 3;
//FROM https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/src/components/subqueries/storage/tests.rs#L87C29-L87C33
pub const KECCAK_F_CAPACITY: usize = 1200;
//NOTE we only do a single storage proof at a time
pub const STORAGE_CAPACITY: usize = 1;
//NOTE we only do a single storage proof at a time
pub const ACCOUNT_CAPACITY: usize = 1;
//hardended GUESS
pub const HEADER_CAPACITY: usize = 1;//132;//33;//132;//1;//132;
//GUESS
pub const RESULTS_CAPACITY: usize = 3;
//FROM https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/src/subquery_aggregation/types.rs#L76
pub const SUBQUERY_AGGREGATION_AGG_VKEY_HASH_IDX: usize = 1;
//FROM https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/configs/production/all_large.yml#L70
pub const MAX_EXTRA_DATA_BYTES: usize = 32;
// //FROM https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/src/components/results/tests.rs#L280
// pub const COMPONENT_CAPACITY_TOTAL: usize = 32;
//FROM https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/src/components/subqueries/block_header/mod.rs#L46
pub const MMR_MAX_NUM_PEAKS: usize = 32; // assuming block number stays in u32, < 2^32
// Merkle Mountain Range size for a tree range with one leaf
pub const MMR_SIZE_1: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3];
// 32 zero bytes
pub const ZERO_32: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
