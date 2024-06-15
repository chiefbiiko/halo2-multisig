use std::{
    fs::File,
    io::{Read, Write},
};

use crate::{constants::*, subquery_aggregation::InputSubqueryAggregation};
use axiom_eth::{
    halo2_base::{
        halo2_proofs::{
            halo2curves::bn256::{Bn256, Fr},
            poly::kzg::commitment::ParamsKZG,
        },
        utils::fs::gen_srs,
    },
    snark_verifier_sdk::{halo2::gen_snark_shplonk, CircuitExt},
    utils::{
        build_utils::{aggregation::get_dummy_aggregation_params, pinning::{
            aggregation::AggregationCircuitPinning, CircuitPinningInstructions, Halo2CircuitPinning, PinnableCircuit, RlcCircuitPinning
        }},
        merkle_aggregation::InputMerkleAggregation,
        snark_verifier::{
            get_accumulator_indices, AggregationCircuitParams, EnhancedSnark,
            NUM_FE_ACCUMULATOR,
        },
    },
};

fn generate_snark<
    C: CircuitExt<Fr> + PinnableCircuit<Pinning = RlcCircuitPinning>,
>(
    name: &'static str,
    params: &ParamsKZG<Bn256>,
    keygen_circuit: C,
    load_prover_circuit: &impl Fn(RlcCircuitPinning) -> C,
) -> anyhow::Result<EnhancedSnark> {
    let cargo_manifest_dir = env!("CARGO_MANIFEST_DIR");
    let pinning_path = format!("{cargo_manifest_dir}/configs/test/{name}.json");
    let pk_path = format!("{cargo_manifest_dir}/data/test/{name}.pk");
    let (pk, pinning) =
        keygen_circuit.create_pk(params, pk_path, pinning_path)?;
    let vk = pk.get_vk();
    let mut vk_file = File::create(format!("data/test/{name}.vk"))?;
    vk.write(&mut vk_file, axiom_eth::halo2_proofs::SerdeFormat::RawBytes)?;
    let mut vk_file = File::create(format!("data/test/{name}.vk.txt"))?;
    write!(vk_file, "{:?}", vk.pinned())?;

    let component_circuit = load_prover_circuit(pinning);

    let snark_path = format!("data/test/{name}.snark");
    let snark =
        gen_snark_shplonk(params, &pk, component_circuit, Some(snark_path));
    Ok(EnhancedSnark { inner: snark, agg_vk_hash_idx: None })
}

fn main() {
    // https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/src/subquery_aggregation/tests.rs#L150

    //TODO pining for subq aggr circuit
    // type CircuitParams = AggregationConfigParams;
    // type BreakPoints = MultiPhaseThreadBreakPoints;
    let subq_aggr_params = get_dummy_aggregation_params(K);
    let break_points = TODO;
    let subq_aggr_pinning = AggregationCircuitPinning::new(subq_aggr_params, break_points);

    // kzg params for subq aggr circuit
    let kzg_params = gen_srs(K.try_into().unwrap());

    //TODO let snark_storage, snark_account = ...

    let subq_aggr_circuit = InputSubqueryAggregation {
        snark_header: header_snark,        //NEEDED?
        snark_results_root: results_snark, //NEEDED?
        snark_account: None,               //TODO
        snark_storage: None,               //TODO
        snark_solidity_mapping: None,
        snark_tx: None,
        snark_receipt: None,
        promise_commit_keccak: keccak_commit, //TODO
    }
    .prover_circuit(subq_aggr_pinning, &kzg_params)
    .expect("subquery aggregation circuit");

    //CircuitBuilderStage::Keygen or Prove

    // let enhanced_snark = EnhancedSnark::new( , None);
    // let input_merkle_aggr = InputMerkleAggregation::new(vec![enhanced_snark]);
    // let aggr_circuit = input_merkle_aggr.prover_circuit(pinning, kzg_params);

    //SUBQUERY AGGREVGATION TESTS
    //   https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/src/subquery_aggregation/tests.rs#L306

    // let snark = gen_snark_shplonk(&params, &pk, prover_circuit, Some(snark_path));
    // let k = 20u32;
    // let params = gen_srs(k);
}

// https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/src/subquery_aggregation/circuit.rs

// https://github.com/search?q=repo%3Aaxiom-crypto%2Faxiom-eth%20prover_circuit&type=code
// let component_circuit = load_prover_circuit(pinning);
// let mut prover_circuit = input.build(CircuitBuilderStage::Prover, pinning.params, &params)?;
// prover_circuit.set_break_points(pinning.break_points);
// let snark = gen_snark_shplonk(&params, &pk, prover_circuit, None::<&str>);
