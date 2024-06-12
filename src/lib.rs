use halo2_base::{
    // gates::{ GateChip, GateInstructions, RangeChip, RangeInstructions },
    // halo2_proofs::halo2curves::{ bn256::Fr, secp256k1::{ Fp, Fq, Secp256k1Affine } },
    // poseidon::hasher::PoseidonHasher,
    // utils::BigPrimeField,
    AssignedValue,
    Context,
    // QuantumCell,
  };
  use axiom_eth::{
    Field,
    storage::circuit::EthStorageInput,
    providers::storage::json_to_mpt_input,
    storage::EthStorageChip,
    utils::component::utils::create_hasher as create_poseidon_hasher
};
use ethers_core::types::{EIP1186ProofResponse, Block, H256};

/// https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/configs/production/all_max.yml#L91
const ACCOUNT_PROOF_MAX_DEPTH: usize = 14;
/// https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/configs/production/all_max.yml#L116
const STORAGE_PROOF_MAX_DEPTH: usize = 13;

// const STATE_ROOT_INDEX: usize = 3;

pub fn json_to_input(block: Block<H256>, proof: EIP1186ProofResponse) -> EthStorageInput {
    let mut input = json_to_mpt_input(proof, ACCOUNT_PROOF_MAX_DEPTH, STORAGE_PROOF_MAX_DEPTH);
    input.acct_pf.root_hash = block.state_root;
    input
}

//WIP
pub fn verify_eip1186<F: Field>(
    ctx: &mut Context<F>,
    chip: &EthStorageChip<F>,
    input: EthStorageInput
) {

//////// TODO refactor :: this is just the circuit beginning
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
