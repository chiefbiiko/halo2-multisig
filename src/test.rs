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
        gates::{
            circuit::builder::BaseCircuitBuilder, GateInstructions, RangeChip,
        },
        halo2_proofs::halo2curves::bn256::Fr,
        safe_types::{
            FixLenBytes, SafeAddress, SafeBytes32, SafeType, SafeTypeChip,
        },
        utils::BigPrimeField,
        AssignedValue, Context,
    },
    keccak::{types::ComponentTypeKeccak, KeccakChip},
    mpt::{MPTChip, MPTProofWitness},
    providers::storage::json_to_mpt_input,
    rlc::circuit::builder::RlcCircuitBuilder,
    rlp::{
        types::{RlpArrayWitness, RlpFieldWitness},
        RlpChip,
    },
    storage::{circuit::EthStorageInput, EthAccountTrace, EthStorageTrace},
    storage::{EthStorageChip, ACCOUNT_STATE_FIELDS_MAX_BYTES},
    utils::{
        circuit_utils::bytes::safe_bytes32_to_hi_lo,
        circuit_utils::bytes::{pack_bytes_to_hilo, unsafe_mpt_root_to_hi_lo},
        component::utils::create_hasher as create_poseidon_hasher,
        component::{
            promise_collector::{PromiseCaller, PromiseCollector},
            ComponentType,
        },
        constrain_vec_equal, encode_addr_to_field,
        hilo::HiLo,
        unsafe_bytes_to_assigned,
    },
    zkevm_hashes::util::eth_types::ToBigEndian,
    Field,
};
use std::sync::{Arc, Mutex};
//   use crate::{verify_eip1186,utils::{test_fixture, rlc_builderz}, constants::*};

//   #[tokio::test]
//   async fn test_verify_eip1186() {
//     // // Inputs
//     // let msg_str =
//     //   b"vulputate ut pharetra tis amet aliquam id diam maecenas ultricies mi eget mauris pharetra et adasdds";

//     let input = test_fixture().await.expect("fixture");

//     let (mut builder1, mut builder2) = rlc_builderz::<Fr>();
//     let promise_caller = PromiseCaller::new(Arc::new(Mutex::new(PromiseCollector::new(vec![
//         ComponentTypeKeccak::<Fr>::get_type_id(),
//     ]))));
//     let range = RangeChip::new(LOOKUP_BITS, builder1.base.lookup_manager().clone());
//     let keccak =
//         KeccakChip::new_with_promise_collector(range.clone(), promise_caller.clone());
//     let rlp = RlpChip::new(&range, None);
//     let mpt = MPTChip::new(rlp, &keccak);
//     let chip = EthStorageChip::new(&mpt, None);
//     let ctx = builder1.base.main(0);

//     verify_eip1186::<Fr>(ctx, builder2.rlc_ctx_pair(), promise_caller, &chip, input);

//     // base_test()
//     //   .k(16)
//     //   .lookup_bits(15)
//     //   .expect_satisfied(true)
//     //   .run(|ctx, range| {
//     //     let fp_chip = FpChip::<Fr>::new(range, 88, 3);
//     //     let fq_chip = FqChip::<Fr>::new(range, 88, 3);
//     //     let ecc_chip = EccChip::<Fr, FpChip<Fr>>::new(&fp_chip);

//     //     let mut poseidon_hasher = PoseidonHasher::<Fr, 3, 2>::new(
//     //       OptimizedPoseidonSpec::new::<8, 57, 0>()
//     //     );
//     //     poseidon_hasher.initialize_consts(ctx, range.gate());

//     //     let nullifier = ecc_chip.load_private_unchecked(ctx, (
//     //       test_data.nullifier.0,
//     //       test_data.nullifier.1,
//     //     ));
//     //     let s = fq_chip.load_private(ctx, test_data.s);
//     //     let c = fq_chip.load_private(ctx, test_data.c);
//     //     let pk = ecc_chip.load_private_unchecked(ctx, (test_data.pk.0, test_data.pk.1));
//     //     let m = test_data.m
//     //       .iter()
//     //       .map(|m| ctx.load_witness(*m))
//     //       .collect::<Vec<_>>();

//     //     let plume_input = PlumeInput {
//     //       nullifier,
//     //       s,
//     //       c,
//     //       pk,
//     //       m,
//     //     };

//     //     verify_plume::<Fr>(ctx, &ecc_chip, &poseidon_hasher, 4, 4, plume_input)
//     //   });
//   }
