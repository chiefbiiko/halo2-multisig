// use halo2_base::{
//     // gates::{ GateChip, GateInstructions, RangeChip, RangeInstructions },
//     // gates::circuit::builder::BaseCircuitBuilder,
//     // halo2_proofs::halo2curves::{ bn256::Fr, secp256k1::{ Fp, Fq, Secp256k1Affine } },
//     // poseidon::hasher::PoseidonHasher,
//     // utils::BigPrimeField,
//     // AssignedValue,
//     // Context,
//     // safe_types::SafeTypeChip,
//     // QuantumCell,
//   };
  use axiom_eth::{
    halo2_base::{
        safe_types::{SafeAddress, SafeBytes32, SafeTypeChip, SafeType, FixLenBytes},
        AssignedValue, Context, gates::{RangeChip, GateInstructions, circuit::builder::BaseCircuitBuilder}},
    keccak::{KeccakChip, types::ComponentTypeKeccak},
    rlp::{RlpChip, types::{RlpArrayWitness, RlpFieldWitness},},
    mpt::{MPTChip, MPTProofWitness},
    Field,
    // rlc::circuit::builder::RlcCircuitBuilder,
    storage::circuit::EthStorageInput,
    providers::storage::json_to_mpt_input,
    storage::{EthStorageChip, ACCOUNT_STATE_FIELDS_MAX_BYTES },
    utils::{
        constrain_vec_equal,
        hilo::HiLo,
        circuit_utils::bytes::{unsafe_mpt_root_to_hi_lo, pack_bytes_to_hilo},
        component::{ComponentType, promise_collector::{PromiseCaller, PromiseCollector}},
        encode_addr_to_field,unsafe_bytes_to_assigned, circuit_utils::bytes::safe_bytes32_to_hi_lo, component::utils::create_hasher as create_poseidon_hasher},
        zkevm_hashes::util::eth_types::ToBigEndian,
};
use axiom_query::{utils::codec::AssignedAccountSubquery, components::subqueries::account::types::{FieldAccountSubqueryCall, ComponentTypeAccountSubquery}};
use ethers_core::types::{EIP1186ProofResponse, Block, H256};
use getset::Getters;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/configs/production/all_max.yml#L91
const ACCOUNT_PROOF_MAX_DEPTH: usize = 14;
/// https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/configs/production/all_max.yml#L116
const STORAGE_PROOF_MAX_DEPTH: usize = 13;
/// The circuit will have 2^k rows.
const K: usize = 10;
/// If you need to use range checks, a good default is to set `lookup_bits` to 1 less than `k`.
const LOOKUP_BITS: usize = K - 1;
/// Constraints are ignored if set to true.
const WITNESS_GEN_ONLY: bool = false;
/// Index of the storage root in an account.
const STORAGE_ROOT_INDEX: usize = 2;

/// This means we can concatenate arrays with individual max length 2^32.
/// https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/src/lib.rs#L23
// pub const DEFAULT_RLC_CACHE_BITS: usize = 32;

// const STATE_ROOT_INDEX: usize = 3;

// pub fn create_ctx_and_chip<'chip, F: Field>() -> ( Box<Context<F>>, EthStorageChip<'chip, F>) {
//     let mut builder:  Box<BaseCircuitBuilder<F>> = Box::new(BaseCircuitBuilder::new(WITNESS_GEN_ONLY).use_k(K));
//     builder.set_lookup_bits(LOOKUP_BITS);
//     let promise_collector = Arc::new(Mutex::new(PromiseCollector::new(vec![
//         ComponentTypeKeccak::<F>::get_type_id(),
//     ])));
//     let range = RangeChip::new(LOOKUP_BITS, builder.lookup_manager().clone());
//     let keccak =
//         KeccakChip::new_with_promise_collector(range.clone(), PromiseCaller::new(promise_collector));
//     let rlp = RlpChip::new(&range, None);
//     let mpt = MPTChip::new(rlp, &keccak);
//     let chip = EthStorageChip::new(&mpt, None);
//     let ctx: Box<Context<F>> = Box::new(builder.main(0).clone());
//     (ctx, chip)
// }

/// Circuit input for a single Storage subquery.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CircuitInputStorageSubquery {
    /// The block number to access the storage state at.
    pub block_number: u64,
    /// Storage proof formatted as MPT input. It will contain the account address.
    /// ### Warning
    /// `proof.acct_pf` will be empty and `proof` will **not** have state_root set.
    pub proof: EthStorageInput,
}

/// Stores storage slot information as well as a proof of inclusion to be verified in parse_storage_phase1. Is returned
/// by `parse_storage_phase0`.
#[derive(Clone, Debug, Getters)]
pub struct EthStorageWitness<F: Field> {
    pub slot: SafeBytes32<F>,
    #[getset(get = "pub")]
    pub(crate) value_witness: RlpFieldWitness<F>,
    #[getset(get = "pub")]
    pub(crate) mpt_witness: MPTProofWitness<F>,
}

/// Stores Account information to be used in later functions. Is returned by `parse_account_proof_phase0`.
#[derive(Clone, Debug, Getters)]
pub struct EthAccountWitness<F: Field> {
    pub address: SafeAddress<F>,
    #[getset(get = "pub")]
    pub(crate) array_witness: RlpArrayWitness<F>,
    #[getset(get = "pub")]
    pub(crate) mpt_witness: MPTProofWitness<F>,
}

/// SafeType for byte (8 bits).
///
/// This is a separate struct from `CompactSafeType` with the same behavior. Because
/// we know only one [`AssignedValue`] is needed to hold the boolean value, we avoid
/// using `CompactSafeType` to avoid the additional heap allocation from a length 1 vector.
#[derive(Clone, Copy, Debug)]
pub struct SafeByte<F: Field>(pub AssignedValue<F>);

pub fn json_to_input(block: Block<H256>, proof: EIP1186ProofResponse) -> EthStorageInput {
    let mut input = json_to_mpt_input(proof, ACCOUNT_PROOF_MAX_DEPTH, STORAGE_PROOF_MAX_DEPTH);
    input.acct_pf.root_hash = block.state_root;
    input
}

//WIP
pub fn verify_eip1186<F: Field>(
    ctx: &mut Context<F>,
    chip: &EthStorageChip<F>,
    input: CircuitInputStorageSubquery,
    promise_caller: PromiseCaller<F>,
) {

        let gate = chip.gate();
    let range = chip.range();
    let safe = SafeTypeChip::new(range);
    // assign address (H160) as single field element
    let addr = ctx.load_witness(encode_addr_to_field(&input.proof.addr));
    // should have already validated input so storage_pfs has length 1
    let (slot, _value, storage_mpt_proof) = input.proof.storage_pfs[0].clone();
    let account_mpt_proof = input.proof.acct_pf.clone();
    // assign `slot` as `SafeBytes32`
    let unsafe_slot = unsafe_bytes_to_assigned(ctx, &slot.to_be_bytes());
    let slot_bytes = safe.raw_bytes_to(ctx, unsafe_slot);
    // convert slot to HiLo to save for later
    let slot = safe_bytes32_to_hi_lo(ctx, gate, &slot_bytes);

    // assign storage proof
    let storage_mpt_proof = storage_mpt_proof.assign(ctx);
    // convert storageRoot from bytes to HiLo for later. `parse_storage_proof` will constrain these witnesses to be bytes
    let storage_root = unsafe_mpt_root_to_hi_lo(ctx, gate, &storage_mpt_proof);

    //START parse_storage_proof_phase0()
    // Check the storage MPT proof
    // let storage_witness = chip.parse_storage_proof_phase0(ctx, slot_bytes, mpt_proof);
    let storage_witness = {
        // parse slot value
        let value_witness =
        chip.rlp().decompose_rlp_field_phase0(ctx, storage_mpt_proof.value_bytes.clone(), 32);
        // check MPT inclusion
        let mpt_witness = chip.mpt.parse_mpt_inclusion_phase0(ctx, storage_mpt_proof);
        EthStorageWitness { slot: slot_bytes, value_witness, mpt_witness }
    };
    //END parse_storage_proof_phase0()

    // Left pad value to 32 bytes and convert to HiLo
    let value = {
        let w = storage_witness.value_witness();
        let inputs = w.field_cells.clone();
        let len = w.field_len;
        let var_len_bytes = SafeTypeChip::unsafe_to_var_len_bytes_vec(inputs, len, 32);
        let fixed_bytes = var_len_bytes.left_pad_to_fixed(ctx, gate);
        pack_bytes_to_hilo(ctx, gate, fixed_bytes.bytes())
    };
    // set slot value to uint256(0) when the slot does not exist in the storage trie
    // let slot_is_empty = storage_witness.mpt_witness().slot_is_empty;
    // let value = HiLo::from_hi_lo(value.hi_lo().map(|x| gate.mul_not(ctx, slot_is_empty, x)));
    //FIXME assert value is 1
   assert_eq!(value.lo().value().get_lower_32(), 1_u32);

    //START parse_account_proof_phase0()
    let account_witness = {
        // assign account proof
        let account_mpt_proof = account_mpt_proof.assign(ctx);
        // parse value RLP([nonce, balance, storage_root, code_hash])
        let array_witness = chip.rlp().decompose_rlp_array_phase0(
            ctx,
            account_mpt_proof.value_bytes.clone(),
            &ACCOUNT_STATE_FIELDS_MAX_BYTES,
            false,
        );
        // Check MPT inclusion for:
        // keccak(addr) => RLP([nonce, balance, storage_root, code_hash])
        let mpt_witness = chip.mpt.parse_mpt_inclusion_phase0(ctx, account_mpt_proof);
        //FIXME just proceed to phase1 storage and account proof handling then see wheteher we can just circumvent safetype address vahalla
        EthAccountWitness { address: SafeType::from(SafeTypeChip::raw_to_fix_len_bytes_vec::<20>::(addr)), array_witness, mpt_witness }
    };
    //END parse_account_proof_phase0()
   
//    

//    PayloadStorageSubquery {
//        storage_witness,
//        storage_root,
//        output: AssignedStorageSubqueryResult {
//            subquery: AssignedStorageSubquery { block_number, addr, slot },
//            value,
//        },
//    }
// }

// let vt = extract_virtual_table(payload.iter().map(|p| p.output));
// let lr: Vec<LogicalResult<F, Self::CompType>> =
//     extract_logical_results(payload.iter().map(|p| p.output));
// let ctx = pool.main();
// // promise calls to header component:
// let account_storage_hash_idx = ctx.load_constant(F::from(STORAGE_ROOT_INDEX as u64));
// for p in payload.iter() {
//     let block_number = p.output.subquery.block_number;
//     let addr = p.output.subquery.addr;
//     let storage_root = p.storage_root;
//     let account_subquery =
//         AssignedAccountSubquery { block_number, addr, field_idx: account_storage_hash_idx };
//     let promise_storage_root = promise_caller
//         .call::<FieldAccountSubqueryCall<F>, ComponentTypeAccountSubquery<F>>(
//             ctx,
//             FieldAccountSubqueryCall(account_subquery),
//         )
//         .unwrap();
//     constrain_vec_equal(ctx, &storage_root.hi_lo(), &promise_storage_root.hi_lo());
// }
// self.payload = Some((keccak, payload));
// CoreBuilderOutput { public_instances: vec![], virtual_table: vt, logical_results: lr }

// #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, Default)]
// pub struct FieldAccountSubquery<T> {
//     pub block_number: T,
//     pub addr: T, // F::CAPACITY >= 160
//     pub field_idx: T,
// }
// type AssignedAccountSubquery<F> = FieldAccountSubquery<AssignedValue<F>>;

let account_storage_hash_idx = ctx.load_constant(F::from(STORAGE_ROOT_INDEX as u64));
// for p in payload.iter() {
    // let block_number = input.block_number;
    let block_number = ctx.load_witness(F::from(input.block_number));
    // let addr = p.output.subquery.addr;
    // let storage_root = p.storage_root;
    let account_subquery =
        AssignedAccountSubquery { block_number, addr, field_idx: account_storage_hash_idx };
    let promise_storage_root = promise_caller
        .call::<FieldAccountSubqueryCall<F>, ComponentTypeAccountSubquery<F>>(
            ctx,
            FieldAccountSubqueryCall(account_subquery),
        )
        .unwrap();
    constrain_vec_equal(ctx, &storage_root.hi_lo(), &promise_storage_root.hi_lo());
// }

// fn virtual_assign_phase1(&mut self, builder: &mut RlcCircuitBuilder<F>) {
//     let (keccak, payload) = self.payload.take().unwrap();
//     // preamble
//     let range_chip = keccak.range();
//     let rlc_chip = builder.rlc_chip(&range_chip.gate);
//     let rlp = RlpChip::new(range_chip, Some(&rlc_chip));
//     let mpt = MPTChip::new(rlp, &keccak);
//     let chip = EthStorageChip::new(&mpt, None);

//     // actual logic
//     builder.parallelize_phase1(payload, |(ctx_gate, ctx_rlc), payload| {
//         handle_single_storage_subquery_phase1((ctx_gate, ctx_rlc), &chip, payload)
//     });

// pub fn handle_single_storage_subquery_phase1<F: Field>(
//    ctx: RlcContextPair<F>,
//    chip: &EthStorageChip<F>,
//    payload: PayloadStorageSubquery<F>,
// ) {
//    chip.parse_storage_proof_phase1(ctx, payload.storage_witness);
// }

    // //TODO parse_storage_proof_phase1()
    //    // Comments below just to log what load_rlc_cache calls are done in the internal functions:
    //     // load_rlc_cache bit_length(2*mpt_witness.key_byte_len)
    //     chip.mpt.parse_mpt_inclusion_phase1((ctx_gate, ctx_rlc), storage_witness.mpt_witness);
    //     // load rlc_cache bit_length(value_witness.rlp_field.len())
    //     let value_trace =
    //         self.rlp().decompose_rlp_field_phase1((ctx_gate, ctx_rlc), witness.value_witness);
    //     let value_trace = value_trace.field_trace;
    //     debug_assert_eq!(value_trace.max_len, 32);
    //     EthStorageTrace { value_trace }











//OLD
// fn virtual_assign_phase0(
//     &mut self,
//     builder: &mut RlcCircuitBuilder<F>,
//     promise_caller: PromiseCaller<F>,
// ) -> CoreBuilderOutput<F, Self::CompType> {
//     // preamble: to be removed
//     let keccak =
//         KeccakChip::new_with_promise_collector(builder.range_chip(), promise_caller.clone());
//     let range_chip = keccak.range();
//     let rlp = RlpChip::new(range_chip, None);
//     let mut poseidon = create_hasher();
//     poseidon.initialize_consts(builder.base.main(0), keccak.gate());

//     // Assumption: we already have input when calling this function.
//     // TODO: automatically derive a dummy input from params.
//     let input = self.input.as_ref().unwrap();

//     let mpt = MPTChip::new(rlp, &keccak);
//     let chip = EthStorageChip::new(&mpt, None);
//     let pool = &mut builder.base.pool(0);
//     // actual logic
//     let payload = parallelize_core(pool, input.requests.clone(), |ctx, subquery| {
//         handle_single_storage_subquery_phase0(ctx, &chip, &subquery)
//     });

//TBC

    //     // assign storage proof
    // let mpt_proof = mpt_proof.assign(ctx);
    // // convert storageRoot from bytes to HiLo for later. `parse_storage_proof` will constrain these witnesses to be bytes
    // let storage_root = unsafe_mpt_root_to_hi_lo(ctx, gate, &mpt_proof);
    // // Check the storage MPT proof
    // let storage_witness = chip.parse_storage_proof_phase0(ctx, slot_bytes, mpt_proof);
    // // Left pad value to 32 bytes and convert to HiLo
    // let value = {
    //     let w = storage_witness.value_witness();
    //     let inputs = w.field_cells.clone();
    //     let len = w.field_len;
    //     let var_len_bytes = SafeTypeChip::unsafe_to_var_len_bytes_vec(inputs, len, 32);
    //     let fixed_bytes = var_len_bytes.left_pad_to_fixed(ctx, gate);
    //     pack_bytes_to_hilo(ctx, gate, fixed_bytes.bytes())
    // };
    // // set slot value to uint256(0) when the slot does not exist in the storage trie
    // let slot_is_empty = storage_witness.mpt_witness().slot_is_empty;
    // let value = HiLo::from_hi_lo(value.hi_lo().map(|x| gate.mul_not(ctx, slot_is_empty, x)));

//WIP handle_single_storage_subquery_phase0
    // /// Assigns `subquery` to virtual cells and then handles the subquery to get result.
    // /// **Assumes** that the storageHash is verified. Returns the assigned private witnesses of
    // /// `(block_number, address, storage_hash)`, to be looked up against Account Component promise.
    // pub fn handle_single_storage_subquery_phase0<F: Field>(
    //     ctx: &mut Context<F>,
    //     chip: &EthStorageChip<F>,
    //     subquery: &CircuitInputStorageSubquery,
    // ) -> PayloadStorageSubquery<F> {
    //     let gate = chip.gate();
    //     let range = chip.range();
    //     let safe = SafeTypeChip::new(range);
    //     // assign address (H160) as single field element
    //     let addr = ctx.load_witness(encode_addr_to_field(&subquery.proof.addr));
    //     // should have already validated input so storage_pfs has length 1
    //     let (slot, _value, mpt_proof) = subquery.proof.storage_pfs[0].clone();
    //     // assign `slot` as `SafeBytes32`
    //     let unsafe_slot = unsafe_bytes_to_assigned(ctx, &slot.to_be_bytes());
    //     let slot_bytes = safe.raw_bytes_to(ctx, unsafe_slot);
    //     // convert slot to HiLo to save for later
    //     let slot = safe_bytes32_to_hi_lo(ctx, gate, &slot_bytes);

    //     // assign storage proof
    //     let mpt_proof = mpt_proof.assign(ctx);
    //     // convert storageRoot from bytes to HiLo for later. `parse_storage_proof` will constrain these witnesses to be bytes
    //     let storage_root = unsafe_mpt_root_to_hi_lo(ctx, gate, &mpt_proof);
    //     // Check the storage MPT proof
    //     let storage_witness = chip.parse_storage_proof_phase0(ctx, slot_bytes, mpt_proof);
    //     // Left pad value to 32 bytes and convert to HiLo
    //     let value = {
    //         let w = storage_witness.value_witness();
    //         let inputs = w.field_cells.clone();
    //         let len = w.field_len;
    //         let var_len_bytes = SafeTypeChip::unsafe_to_var_len_bytes_vec(inputs, len, 32);
    //         let fixed_bytes = var_len_bytes.left_pad_to_fixed(ctx, gate);
    //         pack_bytes_to_hilo(ctx, gate, fixed_bytes.bytes())
    //     };
    //     // set slot value to uint256(0) when the slot does not exist in the storage trie
    //     let slot_is_empty = storage_witness.mpt_witness().slot_is_empty;
    //     let value = HiLo::from_hi_lo(value.hi_lo().map(|x| gate.mul_not(ctx, slot_is_empty, x)));

    //     let block_number = ctx.load_witness(F::from(subquery.block_number));

    //     PayloadStorageSubquery {
    //         storage_witness,
    //         storage_root,
    //         output: AssignedStorageSubqueryResult {
    //             subquery: AssignedStorageSubquery { block_number, addr, slot },
    //             value,
    //         },
    //     }
    // }

//     let vt = extract_virtual_table(payload.iter().map(|p| p.output));
//     let lr: Vec<LogicalResult<F, Self::CompType>> =
//         extract_logical_results(payload.iter().map(|p| p.output));

//     let ctx = pool.main();
//     // promise calls to header component:
//     let account_storage_hash_idx = ctx.load_constant(F::from(STORAGE_ROOT_INDEX as u64));
//     for p in payload.iter() {
//         let block_number = p.output.subquery.block_number;
//         let addr = p.output.subquery.addr;
//         let storage_root = p.storage_root;
//         let account_subquery =
//             AssignedAccountSubquery { block_number, addr, field_idx: account_storage_hash_idx };
//         let promise_storage_root = promise_caller
//             .call::<FieldAccountSubqueryCall<F>, ComponentTypeAccountSubquery<F>>(
//                 ctx,
//                 FieldAccountSubqueryCall(account_subquery),
//             )
//             .unwrap();
//         constrain_vec_equal(ctx, &storage_root.hi_lo(), &promise_storage_root.hi_lo());
//     }
//     self.payload = Some((keccak, payload));
//     CoreBuilderOutput { public_instances: vec![], virtual_table: vt, logical_results: lr }
// }
////////

}
