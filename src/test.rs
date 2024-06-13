
//   use halo2_base::{
//     gates::RangeInstructions,
//     halo2_proofs::halo2curves::bn256::Fr,
//     poseidon::hasher::{ spec::OptimizedPoseidonSpec, PoseidonHasher },
//     utils::testing::base_test,
//   };
//   use halo2_ecc::{ ecc::EccChip, fields::FieldChip, secp256k1::{ FpChip, FqChip } };

//   use crate::{ utils::generate_test_data, PlumeInput };

use axiom_eth::{
    halo2_base::{
        safe_types::{SafeAddress, SafeBytes32, SafeTypeChip, SafeType, FixLenBytes},
        AssignedValue, Context, gates::{RangeChip, GateInstructions, circuit::builder::BaseCircuitBuilder}},
    keccak::{KeccakChip, types::ComponentTypeKeccak},
    rlp::{RlpChip, types::{RlpArrayWitness, RlpFieldWitness},},
    mpt::{MPTChip, MPTProofWitness},
    Field,
    rlc::circuit::builder::RlcCircuitBuilder,
    storage::{circuit::EthStorageInput, EthStorageTrace, EthAccountTrace},
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

  use crate::{verify_eip1186,utils::test_fixture};

  #[tokio::test]
  async fn test_verify_eip1186() {
    // // Inputs
    // let msg_str =
    //   b"vulputate ut pharetra tis amet aliquam id diam maecenas ultricies mi eget mauris pharetra et adasdds";

    let test_data = test_fixture().await.expect("test fixture");

    // base_test()
    //   .k(16)
    //   .lookup_bits(15)
    //   .expect_satisfied(true)
    //   .run(|ctx, range| {
    //     let fp_chip = FpChip::<Fr>::new(range, 88, 3);
    //     let fq_chip = FqChip::<Fr>::new(range, 88, 3);
    //     let ecc_chip = EccChip::<Fr, FpChip<Fr>>::new(&fp_chip);

    //     let mut poseidon_hasher = PoseidonHasher::<Fr, 3, 2>::new(
    //       OptimizedPoseidonSpec::new::<8, 57, 0>()
    //     );
    //     poseidon_hasher.initialize_consts(ctx, range.gate());

    //     let nullifier = ecc_chip.load_private_unchecked(ctx, (
    //       test_data.nullifier.0,
    //       test_data.nullifier.1,
    //     ));
    //     let s = fq_chip.load_private(ctx, test_data.s);
    //     let c = fq_chip.load_private(ctx, test_data.c);
    //     let pk = ecc_chip.load_private_unchecked(ctx, (test_data.pk.0, test_data.pk.1));
    //     let m = test_data.m
    //       .iter()
    //       .map(|m| ctx.load_witness(*m))
    //       .collect::<Vec<_>>();

    //     let plume_input = PlumeInput {
    //       nullifier,
    //       s,
    //       c,
    //       pk,
    //       m,
    //     };

    //     verify_plume::<Fr>(ctx, &ecc_chip, &poseidon_hasher, 4, 4, plume_input)
    //   });
  }
