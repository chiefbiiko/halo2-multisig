use axiom_eth::{
    halo2_base::{
        gates::{
            circuit::builder::BaseCircuitBuilder,
            flex_gate::threads::parallelize_core, GateInstructions, RangeChip,
        },
        safe_types::{
            FixLenBytes, SafeAddress, SafeBytes32, SafeType, SafeTypeChip,
        },
        AssignedValue, Context,
    },
    halo2_proofs::plonk::ConstraintSystem,
    keccak::{types::ComponentTypeKeccak, KeccakChip},
    mpt::{MPTChip, MPTProofWitness},
    providers::storage::json_to_mpt_input,
    rlc::circuit::builder::{RlcCircuitBuilder, RlcContextPair},
    rlp::{
        types::{RlpArrayWitness, RlpFieldWitness},
        RlpChip,
    },
    storage::{
        circuit::EthStorageInput, EthAccountTrace, EthStorageChip,
        EthStorageTrace, EthStorageWitness, ACCOUNT_STATE_FIELDS_MAX_BYTES,
    },
    utils::{
        build_utils::aggregation::CircuitMetadata,
        circuit_utils::bytes::{
            pack_bytes_to_hilo, safe_bytes32_to_hi_lo, unsafe_mpt_root_to_hi_lo,
        },
        component::{
            circuit::{
                ComponentBuilder, ComponentCircuitImpl, CoreBuilder,
                CoreBuilderOutput, CoreBuilderOutputParams, CoreBuilderParams,
            },
            promise_collector::{PromiseCaller, PromiseCollector},
            promise_loader::{
                combo::PromiseBuilderCombo, single::PromiseLoader,
            },
            types::LogicalEmpty,
            utils::create_hasher,
            ComponentType, LogicalResult,
        },
        constrain_vec_equal, encode_addr_to_field,
        hilo::HiLo,
        unsafe_bytes_to_assigned,
    },
    zkevm_hashes::util::eth_types::ToBigEndian,
    Field,
};
use axiom_query::{
    components::subqueries::{
        account::{
            types::{ComponentTypeAccountSubquery, FieldAccountSubqueryCall},
            STORAGE_ROOT_INDEX,
        },
        storage::types::{
            CircuitInputStorageShard, CircuitInputStorageSubquery,
            ComponentTypeStorageSubquery,
        },
        // common::{extract_logical_results, extract_virtual_table},
    },
    utils::codec::{
        AssignedAccountSubquery, AssignedStorageSubquery,
        AssignedStorageSubqueryResult,
    },
};
use ethers_core::types::{Block, EIP1186ProofResponse, H256};
// use getset::Getters;
use serde::{Deserialize, Serialize};
// use std::sync::{Arc, Mutex};

// mod constants;
// #[cfg(test)]
// mod test;
// // mod types;
// mod utils;

use crate::utils::{extract_logical_results, extract_virtual_table};

// use utils::json_to_input;

// use constants::*;
// use types::{
//     // CircuitInputStorageSubquery,
//      EthAccountWitness, EthStorageWitness,
// };

use axiom_query::components::subqueries::storage::circuit::CoreParamsStorageSubquery;

pub struct CoreBuilderStorageSubquery<F: Field> {
    input: Option<CircuitInputStorageShard<F>>,
    params: CoreParamsStorageSubquery,
    payload: Option<(KeccakChip<F>, Vec<PayloadStorageSubquery<F>>)>,
}

// /// Specify the output format of StorageSubquery component.
// #[derive(Clone, Default, Serialize, Deserialize)]
// pub struct CoreParamsStorageSubquery {
//     /// The maximum number of subqueries of this type allowed in a single circuit.
//     pub capacity: usize,
//     /// The maximum depth of the storage MPT trie supported by this circuit.
//     /// The depth is defined as the maximum length of an storage proof, where the storage proof always ends in a terminal leaf node.
//     ///
//     /// In production this will be set to 13 based on the MPT analysis from https://hackmd.io/@axiom/BJBledudT
//     pub max_trie_depth: usize,
// }
// impl CoreBuilderParams for CoreParamsStorageSubquery {
//     fn get_output_params(&self) -> CoreBuilderOutputParams {
//         CoreBuilderOutputParams::new(vec![self.capacity])
//     }
// }

type CKeccak<F> = ComponentTypeKeccak<F>;
type CAccount<F> = ComponentTypeAccountSubquery<F>;
/// Used for loading storage promise results.
pub type PromiseLoaderStorageSubquery<F> = PromiseBuilderCombo<
    F,
    PromiseLoader<F, CKeccak<F>>,
    PromiseLoader<F, CAccount<F>>,
>;
pub type ComponentCircuitStorageSubquery<F> = ComponentCircuitImpl<
    F,
    CoreBuilderStorageSubquery<F>,
    PromiseLoaderStorageSubquery<F>,
>;

impl<F: Field> CircuitMetadata for CoreBuilderStorageSubquery<F> {
    const HAS_ACCUMULATOR: bool = false;
    fn num_instance(&self) -> Vec<usize> {
        unreachable!()
    }
}

impl<F: Field> ComponentBuilder<F> for CoreBuilderStorageSubquery<F> {
    type Params = CoreParamsStorageSubquery;

    fn new(params: Self::Params) -> Self {
        Self { input: None, params, payload: None }
    }
    fn get_params(&self) -> Self::Params {
        self.params.clone()
    }
    fn clear_witnesses(&mut self) {
        self.payload = None;
    }
    fn calculate_params(&mut self) -> Self::Params {
        self.params.clone()
    }
    fn configure_with_params(_: &mut ConstraintSystem<F>, _: Self::Params) {}
}

impl<F: Field> CoreBuilder<F> for CoreBuilderStorageSubquery<F> {
    type CompType = ComponentTypeStorageSubquery<F>;
    type PublicInstanceValue = LogicalEmpty<F>;
    type PublicInstanceWitness = LogicalEmpty<AssignedValue<F>>;
    type CoreInput = CircuitInputStorageShard<F>;

    fn feed_input(&mut self, input: Self::CoreInput) -> anyhow::Result<()> {
        for r in &input.requests {
            if r.proof.storage_pfs.len() != 1 {
                anyhow::bail!(
                    "InvalidInput: each storage proof input must have exactly one storage proof"
                );
            }
            if r.proof.storage_pfs[0].2.max_depth != self.params.max_trie_depth
            {
                anyhow::bail!("StorageSubquery: request MPT max depth {} does not match configured max depth {}", r.proof.storage_pfs[0].2.max_depth, self.params.max_trie_depth);
            }
        }
        self.input = Some(input);
        Ok(())
    }
    fn virtual_assign_phase0(
        &mut self,
        builder: &mut RlcCircuitBuilder<F>,
        promise_caller: PromiseCaller<F>,
    ) -> CoreBuilderOutput<F, Self::CompType> {
        // preamble: to be removed
        let keccak = KeccakChip::new_with_promise_collector(
            builder.range_chip(),
            promise_caller.clone(),
        );
        let range_chip = keccak.range();
        let rlp = RlpChip::new(range_chip, None);
        let mut poseidon = create_hasher();
        poseidon.initialize_consts(builder.base.main(0), keccak.gate());

        // Assumption: we already have input when calling this function.
        // TODO: automatically derive a dummy input from params.
        let input = self.input.as_ref().unwrap();

        let mpt = MPTChip::new(rlp, &keccak);
        let chip = EthStorageChip::new(&mpt, None);
        let pool = &mut builder.base.pool(0);
        // actual logic
        let payload =
            parallelize_core(pool, input.requests.clone(), |ctx, subquery| {
                handle_single_storage_subquery_phase0(ctx, &chip, &subquery)
            });

        let vt = extract_virtual_table(payload.iter().map(|p| p.output));
        let lr: Vec<LogicalResult<F, Self::CompType>> =
            extract_logical_results(payload.iter().map(|p| p.output));

        let ctx = pool.main();
        // promise calls to header component:
        let account_storage_hash_idx =
            ctx.load_constant(F::from(STORAGE_ROOT_INDEX as u64));
        for p in payload.iter() {
            let block_number = p.output.subquery.block_number;
            let addr = p.output.subquery.addr;
            let storage_root = p.storage_root;
            let account_subquery = AssignedAccountSubquery {
                block_number,
                addr,
                field_idx: account_storage_hash_idx,
            };
            let promise_storage_root = promise_caller
                .call::<FieldAccountSubqueryCall<F>, ComponentTypeAccountSubquery<F>>(
                    ctx,
                    FieldAccountSubqueryCall(account_subquery),
                )
                .unwrap();
            constrain_vec_equal(
                ctx,
                &storage_root.hi_lo(),
                &promise_storage_root.hi_lo(),
            );
        }
        self.payload = Some((keccak, payload));
        CoreBuilderOutput {
            public_instances: vec![],
            virtual_table: vt,
            logical_results: lr,
        }
    }

    fn virtual_assign_phase1(&mut self, builder: &mut RlcCircuitBuilder<F>) {
        let (keccak, payload) = self.payload.take().unwrap();
        // preamble
        let range_chip = keccak.range();
        let rlc_chip = builder.rlc_chip(&range_chip.gate);
        let rlp = RlpChip::new(range_chip, Some(&rlc_chip));
        let mpt = MPTChip::new(rlp, &keccak);
        let chip = EthStorageChip::new(&mpt, None);

        // actual logic
        builder.parallelize_phase1(payload, |(ctx_gate, ctx_rlc), payload| {
            handle_single_storage_subquery_phase1(
                (ctx_gate, ctx_rlc),
                &chip,
                payload,
            )
        });
    }
}

pub struct PayloadStorageSubquery<F: Field> {
    pub storage_witness: EthStorageWitness<F>,
    pub storage_root: HiLo<AssignedValue<F>>,
    pub output: AssignedStorageSubqueryResult<F>,
}

/// Assigns `subquery` to virtual cells and then handles the subquery to get result.
/// **Assumes** that the storageHash is verified. Returns the assigned private witnesses of
/// `(block_number, address, storage_hash)`, to be looked up against Account Component promise.
pub fn handle_single_storage_subquery_phase0<F: Field>(
    ctx: &mut Context<F>,
    chip: &EthStorageChip<F>,
    subquery: &CircuitInputStorageSubquery,
) -> PayloadStorageSubquery<F> {
    let gate = chip.gate();
    let range = chip.range();
    let safe = SafeTypeChip::new(range);
    // assign address (H160) as single field element
    let addr = ctx.load_witness(encode_addr_to_field(&subquery.proof.addr));
    // should have already validated input so storage_pfs has length 1
    let (slot, _value, mpt_proof) = subquery.proof.storage_pfs[0].clone();
    // assign `slot` as `SafeBytes32`
    let unsafe_slot = unsafe_bytes_to_assigned(ctx, &slot.to_be_bytes());
    let slot_bytes = safe.raw_bytes_to(ctx, unsafe_slot);
    // convert slot to HiLo to save for later
    let slot = safe_bytes32_to_hi_lo(ctx, gate, &slot_bytes);

    // assign storage proof
    let mpt_proof = mpt_proof.assign(ctx);
    // convert storageRoot from bytes to HiLo for later. `parse_storage_proof` will constrain these witnesses to be bytes
    let storage_root = unsafe_mpt_root_to_hi_lo(ctx, gate, &mpt_proof);
    // Check the storage MPT proof
    let storage_witness =
        chip.parse_storage_proof_phase0(ctx, slot_bytes, mpt_proof);
    // Left pad value to 32 bytes and convert to HiLo
    let value = {
        let w = storage_witness.value_witness();
        let inputs = w.field_cells.clone();
        let len = w.field_len;
        let var_len_bytes =
            SafeTypeChip::unsafe_to_var_len_bytes_vec(inputs, len, 32);
        let fixed_bytes = var_len_bytes.left_pad_to_fixed(ctx, gate);
        pack_bytes_to_hilo(ctx, gate, fixed_bytes.bytes())
    };
    // set slot value to uint256(0) when the slot does not exist in the storage trie
    let slot_is_empty = storage_witness.mpt_witness().slot_is_empty;
    let value = HiLo::from_hi_lo(
        value.hi_lo().map(|x| gate.mul_not(ctx, slot_is_empty, x)),
    );

    let block_number = ctx.load_witness(F::from(subquery.block_number));

    PayloadStorageSubquery {
        storage_witness,
        storage_root,
        output: AssignedStorageSubqueryResult {
            subquery: AssignedStorageSubquery { block_number, addr, slot },
            value,
        },
    }
}

pub fn handle_single_storage_subquery_phase1<F: Field>(
    ctx: RlcContextPair<F>,
    chip: &EthStorageChip<F>,
    payload: PayloadStorageSubquery<F>,
) {
    chip.parse_storage_proof_phase1(ctx, payload.storage_witness);
}

// //OLD
// pub fn verify_eip1186<F: Field>(
//     ctx: &mut Context<F>,
//     (ctx_gate, ctx_rlc): RlcContextPair<F>,
//     promise_caller: PromiseCaller<F>,
//     chip: &EthStorageChip<F>,
//     input: CircuitInputStorageSubquery,
//     // mut builder: RlcCircuitBuilder<F>,
// ) {

//         let gate = chip.gate();
//     let range = chip.range();
//     let safe = SafeTypeChip::new(range);
//     // assign address (H160) as single field element
//     let addr = ctx.load_witness(encode_addr_to_field(&input.proof.addr));
//     // should have already validated input so storage_pfs has length 1
//     let (slot, _value, storage_mpt_proof) = input.proof.storage_pfs[0].clone();
//     let account_mpt_proof = input.proof.acct_pf.clone();
//     // assign `slot` as `SafeBytes32`
//     let unsafe_slot = unsafe_bytes_to_assigned(ctx, &slot.to_be_bytes());
//     let slot_bytes = safe.raw_bytes_to(ctx, unsafe_slot);
//     // convert slot to HiLo to save for later
//     let slot = safe_bytes32_to_hi_lo(ctx, gate, &slot_bytes);

//     // assign storage proof
//     let storage_mpt_proof = storage_mpt_proof.assign(ctx);
//     // convert storageRoot from bytes to HiLo for later. `parse_storage_proof` will constrain these witnesses to be bytes
//     let storage_root = unsafe_mpt_root_to_hi_lo(ctx, gate, &storage_mpt_proof);

//     //START parse_storage_proof_phase0()
//     // Check the storage MPT proof
//     // let storage_witness = chip.parse_storage_proof_phase0(ctx, slot_bytes, mpt_proof);
//     let storage_witness = {
//         // parse slot value
//         let value_witness =
//         chip.rlp().decompose_rlp_field_phase0(ctx, storage_mpt_proof.value_bytes.clone(), 32);
//         // check MPT inclusion
//         let mpt_witness = chip.mpt.parse_mpt_inclusion_phase0(ctx, storage_mpt_proof);
//         EthStorageWitness { slot: slot_bytes, value_witness, mpt_witness }
//     };
//     //END parse_storage_proof_phase0()

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
//     // let slot_is_empty = storage_witness.mpt_witness().slot_is_empty;
//     // let value = HiLo::from_hi_lo(value.hi_lo().map(|x| gate.mul_not(ctx, slot_is_empty, x)));
//     // assert value is 1 aka Safe signed msg
//    let lo_bytes: [u8; 16] = value.lo().value().to_bytes_le()[0..16].try_into().expect("lo bytes");
//    let hi_bytes: [u8; 16] = value.hi().value().to_bytes_le()[0..16].try_into().expect("hi bytes");
//    assert_eq!(u128::from_le_bytes(lo_bytes) + u128::from_le_bytes(hi_bytes), 1_u128);//FIXME
//    //ctx.constrain_equal //FIXME

//     //WIP TODO dodge ComponentTypeAccountSubquery and rather constrain storage_root == account_array_witness[STORAGE_ROOT_INDEX]

// //    let account_storage_hash_idx = ctx.load_constant(F::from(STORAGE_ROOT_INDEX as u64));
// //    // for p in payload.iter() {
// //        // let block_number = input.block_number;
// //        let block_number = ctx.load_witness(F::from(input.block_number));
// //        // let addr = p.output.subquery.addr;
// //        // let storage_root = p.storage_root;
// //        let account_subquery =
// //            AssignedAccountSubquery { block_number, addr, field_idx: account_storage_hash_idx };
// //        let promise_storage_root = promise_caller
// //            .call::<FieldAccountSubqueryCall<F>, ComponentTypeAccountSubquery<F>>(
// //                ctx,
// //                FieldAccountSubqueryCall(account_subquery),
// //            )
// //            .unwrap();
// //        constrain_vec_equal(ctx, &storage_root.hi_lo(), &promise_storage_root.hi_lo());

//     //START parse_account_proof_phase0()
//     let account_witness = {
//         // assign account proof
//         let account_mpt_proof = account_mpt_proof.assign(ctx);
//         // parse value RLP([nonce, balance, storage_root, code_hash])
//         let array_witness = chip.rlp().decompose_rlp_array_phase0(
//             ctx,
//             account_mpt_proof.value_bytes.clone(),
//             &ACCOUNT_STATE_FIELDS_MAX_BYTES,
//             false,
//         );
//         // Check MPT inclusion for:
//         // keccak(addr) => RLP([nonce, balance, storage_root, code_hash])
//         let mpt_witness = chip.mpt.parse_mpt_inclusion_phase0(ctx, account_mpt_proof);

//         let unsafe_addr = unsafe_bytes_to_assigned(ctx, &input.proof.addr.to_fixed_bytes());
//         let addr_bytes = safe.raw_bytes_to(ctx, unsafe_addr);

//         EthAccountWitness { address: addr_bytes, array_witness, mpt_witness }
//     };
//     //END parse_account_proof_phase0()

// println!(">>>>>>>>>>>>> completed phase 0");

// // fn virtual_assign_phase1(&mut self, builder: &mut RlcCircuitBuilder<F>) {
// //     let (keccak, payload) = self.payload.take().unwrap();
// //     // preamble
// //     let range_chip = keccak.range();
// //     let rlc_chip = builder.rlc_chip(&range_chip.gate);
// //     let rlp = RlpChip::new(range_chip, Some(&rlc_chip));
// //     let mpt = MPTChip::new(rlp, &keccak);
// //     let chip = EthStorageChip::new(&mpt, None);

// //     // actual logic
// //     builder.parallelize_phase1(payload, |(ctx_gate, ctx_rlc), payload| {
// //         handle_single_storage_subquery_phase1((ctx_gate, ctx_rlc), &chip, payload)
// //     });

// // pub fn handle_single_storage_subquery_phase1<F: Field>(
// //    ctx: RlcContextPair<F>,
// //    chip: &EthStorageChip<F>,
// //    payload: PayloadStorageSubquery<F>,
// // ) {
// //    chip.parse_storage_proof_phase1(ctx, payload.storage_witness);
// // }

//     //START parse_storage_proof_phase1()
//     let storage_trace = {
//         // Comments below just to log what load_rlc_cache calls are done in the internal functions:
//         // load_rlc_cache bit_length(2*mpt_witness.key_byte_len)
//         chip.mpt.parse_mpt_inclusion_phase1((ctx_gate, ctx_rlc), storage_witness.mpt_witness);
//         // load rlc_cache bit_length(value_witness.rlp_field.len())
//         let value_trace =
//             chip.rlp().decompose_rlp_field_phase1((ctx_gate, ctx_rlc), storage_witness.value_witness);
//         let value_trace = value_trace.field_trace;
//         debug_assert_eq!(value_trace.max_len, 32);
//         EthStorageTrace { value_trace }
//     };
//     //END parse_storage_proof_phase1()

//     //START parse_account_proof_phase1()
//     let account_trace = {
//                 // Comments below just to log what load_rlc_cache calls are done in the internal functions:
//         // load_rlc_cache bit_length(2*mpt_witness.key_byte_len)
//         chip.mpt.parse_mpt_inclusion_phase1((ctx_gate, ctx_rlc), account_witness.mpt_witness);
//         // load rlc_cache bit_length(array_witness.rlp_array.len())
//         let array_trace: [_; 4] = chip
//             .rlp()
//             .decompose_rlp_array_phase1((ctx_gate, ctx_rlc), account_witness.array_witness, false)
//             .field_trace
//             .try_into()
//             .unwrap();
//         let [nonce_trace, balance_trace, storage_root_trace, code_hash_trace] =
//             array_trace.map(|trace| trace.field_trace);
//         EthAccountTrace { nonce_trace, balance_trace, storage_root_trace, code_hash_trace }
//     };
//     //END parse_account_proof_phase1()

// //OLD
// // fn virtual_assign_phase0(
// //     &mut self,
// //     builder: &mut RlcCircuitBuilder<F>,
// //     promise_caller: PromiseCaller<F>,
// // ) -> CoreBuilderOutput<F, Self::CompType> {
// //     // preamble: to be removed
// //     let keccak =
// //         KeccakChip::new_with_promise_collector(builder.range_chip(), promise_caller.clone());
// //     let range_chip = keccak.range();
// //     let rlp = RlpChip::new(range_chip, None);
// //     let mut poseidon = create_hasher();
// //     poseidon.initialize_consts(builder.base.main(0), keccak.gate());

// //     // Assumption: we already have input when calling this function.
// //     // TODO: automatically derive a dummy input from params.
// //     let input = self.input.as_ref().unwrap();

// //     let mpt = MPTChip::new(rlp, &keccak);
// //     let chip = EthStorageChip::new(&mpt, None);
// //     let pool = &mut builder.base.pool(0);
// //     // actual logic
// //     let payload = parallelize_core(pool, input.requests.clone(), |ctx, subquery| {
// //         handle_single_storage_subquery_phase0(ctx, &chip, &subquery)
// //     });

// //TBC

//     //     // assign storage proof
//     // let mpt_proof = mpt_proof.assign(ctx);
//     // // convert storageRoot from bytes to HiLo for later. `parse_storage_proof` will constrain these witnesses to be bytes
//     // let storage_root = unsafe_mpt_root_to_hi_lo(ctx, gate, &mpt_proof);
//     // // Check the storage MPT proof
//     // let storage_witness = chip.parse_storage_proof_phase0(ctx, slot_bytes, mpt_proof);
//     // // Left pad value to 32 bytes and convert to HiLo
//     // let value = {
//     //     let w = storage_witness.value_witness();
//     //     let inputs = w.field_cells.clone();
//     //     let len = w.field_len;
//     //     let var_len_bytes = SafeTypeChip::unsafe_to_var_len_bytes_vec(inputs, len, 32);
//     //     let fixed_bytes = var_len_bytes.left_pad_to_fixed(ctx, gate);
//     //     pack_bytes_to_hilo(ctx, gate, fixed_bytes.bytes())
//     // };
//     // // set slot value to uint256(0) when the slot does not exist in the storage trie
//     // let slot_is_empty = storage_witness.mpt_witness().slot_is_empty;
//     // let value = HiLo::from_hi_lo(value.hi_lo().map(|x| gate.mul_not(ctx, slot_is_empty, x)));

// //WIP handle_single_storage_subquery_phase0
//     // /// Assigns `subquery` to virtual cells and then handles the subquery to get result.
//     // /// **Assumes** that the storageHash is verified. Returns the assigned private witnesses of
//     // /// `(block_number, address, storage_hash)`, to be looked up against Account Component promise.
//     // pub fn handle_single_storage_subquery_phase0<F: Field>(
//     //     ctx: &mut Context<F>,
//     //     chip: &EthStorageChip<F>,
//     //     subquery: &CircuitInputStorageSubquery,
//     // ) -> PayloadStorageSubquery<F> {
//     //     let gate = chip.gate();
//     //     let range = chip.range();
//     //     let safe = SafeTypeChip::new(range);
//     //     // assign address (H160) as single field element
//     //     let addr = ctx.load_witness(encode_addr_to_field(&subquery.proof.addr));
//     //     // should have already validated input so storage_pfs has length 1
//     //     let (slot, _value, mpt_proof) = subquery.proof.storage_pfs[0].clone();
//     //     // assign `slot` as `SafeBytes32`
//     //     let unsafe_slot = unsafe_bytes_to_assigned(ctx, &slot.to_be_bytes());
//     //     let slot_bytes = safe.raw_bytes_to(ctx, unsafe_slot);
//     //     // convert slot to HiLo to save for later
//     //     let slot = safe_bytes32_to_hi_lo(ctx, gate, &slot_bytes);

//     //     // assign storage proof
//     //     let mpt_proof = mpt_proof.assign(ctx);
//     //     // convert storageRoot from bytes to HiLo for later. `parse_storage_proof` will constrain these witnesses to be bytes
//     //     let storage_root = unsafe_mpt_root_to_hi_lo(ctx, gate, &mpt_proof);
//     //     // Check the storage MPT proof
//     //     let storage_witness = chip.parse_storage_proof_phase0(ctx, slot_bytes, mpt_proof);
//     //     // Left pad value to 32 bytes and convert to HiLo
//     //     let value = {
//     //         let w = storage_witness.value_witness();
//     //         let inputs = w.field_cells.clone();
//     //         let len = w.field_len;
//     //         let var_len_bytes = SafeTypeChip::unsafe_to_var_len_bytes_vec(inputs, len, 32);
//     //         let fixed_bytes = var_len_bytes.left_pad_to_fixed(ctx, gate);
//     //         pack_bytes_to_hilo(ctx, gate, fixed_bytes.bytes())
//     //     };
//     //     // set slot value to uint256(0) when the slot does not exist in the storage trie
//     //     let slot_is_empty = storage_witness.mpt_witness().slot_is_empty;
//     //     let value = HiLo::from_hi_lo(value.hi_lo().map(|x| gate.mul_not(ctx, slot_is_empty, x)));

//     //     let block_number = ctx.load_witness(F::from(subquery.block_number));

//     //     PayloadStorageSubquery {
//     //         storage_witness,
//     //         storage_root,
//     //         output: AssignedStorageSubqueryResult {
//     //             subquery: AssignedStorageSubquery { block_number, addr, slot },
//     //             value,
//     //         },
//     //     }
//     // }

// //     let vt = extract_virtual_table(payload.iter().map(|p| p.output));
// //     let lr: Vec<LogicalResult<F, Self::CompType>> =
// //         extract_logical_results(payload.iter().map(|p| p.output));

// //     let ctx = pool.main();
// //     // promise calls to header component:
// //     let account_storage_hash_idx = ctx.load_constant(F::from(STORAGE_ROOT_INDEX as u64));
// //     for p in payload.iter() {
// //         let block_number = p.output.subquery.block_number;
// //         let addr = p.output.subquery.addr;
// //         let storage_root = p.storage_root;
// //         let account_subquery =
// //             AssignedAccountSubquery { block_number, addr, field_idx: account_storage_hash_idx };
// //         let promise_storage_root = promise_caller
// //             .call::<FieldAccountSubqueryCall<F>, ComponentTypeAccountSubquery<F>>(
// //                 ctx,
// //                 FieldAccountSubqueryCall(account_subquery),
// //             )
// //             .unwrap();
// //         constrain_vec_equal(ctx, &storage_root.hi_lo(), &promise_storage_root.hi_lo());
// //     }
// //     self.payload = Some((keccak, payload));
// //     CoreBuilderOutput { public_instances: vec![], virtual_table: vt, logical_results: lr }
// // }
// ////////

// }
