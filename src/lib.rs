use halo2_base::{
    gates::{ GateChip, GateInstructions, RangeChip, RangeInstructions },
    halo2_proofs::halo2curves::{ bn256::Fr, secp256k1::{ Fp, Fq, Secp256k1Affine } },
    poseidon::hasher::PoseidonHasher,
    utils::BigPrimeField,
    AssignedValue,
    Context,
    QuantumCell,
  };
  use halo2_ecc::{
    bigint::ProperCrtUint,
    // bigint::{ big_is_even, ProperCrtUint },
    // ecc::EcPoint,
    // fields::FieldChip,
    // secp256k1::{ hash_to_curve::{ hash_to_curve, util::fe_to_bytes_le }, Secp256k1Chip },
  };

////////
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

#[derive(Clone, Debug)]
pub struct Eip1186Input<F: BigPrimeField> {
pub safe_address: ProperCrtUint<F>,      // Safe address
pub msg_hash: ProperCrtUint<F>,          // Custom msg hash
pub state_root: ProperCrtUint<F>,        // eth_getBlockBy*::response.stateRoot
pub storage_root: ProperCrtUint<F>,      // eth_getProof::response.storageHash
pub state_trie_key: ProperCrtUint<F>,    // keccak256(safe)
pub storage_trie_key: ProperCrtUint<F>,  // keccak256(msg_hash + uint256(7))
pub account_proof: Vec<Vec<AssignedValue<F>>>, // list of bytes // eth_getProof::response.accountProof
pub storage_proof: Vec<Vec<AssignedValue<F>>>, // list of bytes // eth_getProof::response.storageProof.proof
pub header_rlp: Vec<AssignedValue<F>>, // bytes // RLP-encoded header
}

  pub fn verify_eip1186<F: BigPrimeField>(
    ctx: &mut Context<F>,
    chip: &EthStorageChip<F>,

    // secp256k1_chip: &Secp256k1Chip<'_, F>,
    // poseidon_hasher: &PoseidonHasher<F, 3, 2>,
    // fixed_window_bits: usize,
    // var_window_bits: usize,
    input: Eip1186Input<F>
  ) {





  }