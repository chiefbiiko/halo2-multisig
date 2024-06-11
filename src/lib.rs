use halo2_base::{
    // gates::{ GateChip, GateInstructions, RangeChip, RangeInstructions },
    // halo2_proofs::halo2curves::{ bn256::Fr, secp256k1::{ Fp, Fq, Secp256k1Affine } },
    // poseidon::hasher::PoseidonHasher,
    // utils::BigPrimeField,
    AssignedValue,
    Context,
    // QuantumCell,
  };
  use halo2_ecc::bigint::ProperCrtUint;
  use axiom_eth::{Field,storage::EthStorageChip, utils::component::utils::create_hasher as create_poseidon_hasher};

////////
// /// Circuit input for a single Storage subquery.
// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct CircuitInputStorageSubquery {
//     /// The block number to access the storage state at.
//     pub block_number: u64,
//     /// Storage proof formatted as MPT input. It will contain the account address.
//     /// ### Warning
//     /// `proof.acct_pf` will be empty and `proof` will **not** have state_root set.
//     pub proof: EthStorageInput,
// }

// #[derive(Clone, Debug, Hash, Serialize, Deserialize)]
// pub struct EthStorageInput {
//     pub addr: Address,
//     pub acct_pf: MPTInput,
//     pub acct_state: Vec<Vec<u8>>,
//     /// A vector of (slot, value, proof) tuples
//     pub storage_pfs: Vec<(U256, U256, MPTInput)>,
// }

/// Does **not** perform any range checks on witnesses to check if they are actually bytes.
/// This should be done in the `parse_mpt_inclusion_phase0` function

// #[derive(Clone, Debug, Hash, Serialize, Deserialize)]
// /// The pre-assigned inputs for the MPT proof
// pub struct MPTInput {
//     // claim specification: (path, value)
//     /// A Merkle-Patricia Trie is a mapping `path => value`
//     ///
//     /// As an example, the MPT state trie of Ethereum has
//     /// `path = keccak256(address) => value = rlp(account)`
//     pub path: PathBytes,
//     pub value: Vec<u8>,
//     pub root_hash: H256,
//     /// Inclusion proofs will always end in a terminal node: we extract this terminal node in cases where it was originally embedded inside the last branch node.
//     pub proof: Vec<Vec<u8>>,
//     pub slot_is_empty: bool,
//     pub value_max_byte_len: usize,
//     pub max_depth: usize,
//     pub max_key_byte_len: usize,
//     pub key_byte_len: Option<usize>,
// }
////////

//??? how do we go from rpc1186proofresponse to MPTInput?

//WIP
#[derive(Clone, Debug)]
pub struct Eip1186Input<F: Field> {
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

//WIP
pub fn verify_eip1186<F: Field>(
    ctx: &mut Context<F>,
    chip: &EthStorageChip<F>,
    input: Eip1186Input<F>
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
