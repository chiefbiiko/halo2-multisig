use std::{
    fs::File,
    io::{Read, Write},
};

use crate::{constants::*, subquery_aggregation::InputSubqueryAggregation};
use axiom_eth::{
    halo2_base::{
        gates::circuit::{BaseCircuitParams, CircuitBuilderStage}, halo2_proofs::{
            halo2curves::bn256::{Bn256, Fr},
            poly::kzg::commitment::ParamsKZG,
        }, utils::fs::gen_srs
    }, rlc::circuit::RlcCircuitParams, snark_verifier_sdk::{halo2::{aggregation::AggregationConfigParams, gen_snark_shplonk}, CircuitExt}, utils::{
        build_utils::pinning::{
            aggregation::AggregationCircuitPinning, CircuitPinningInstructions, Halo2CircuitPinning, PinnableCircuit, RlcCircuitPinning
        },
        merkle_aggregation::InputMerkleAggregation,
        snark_verifier::{
            get_accumulator_indices, AggregationCircuitParams, EnhancedSnark,
            NUM_FE_ACCUMULATOR,
        },
    }
};

use axiom_codec::constants::{
        USER_ADVICE_COLS, USER_FIXED_COLS, USER_INSTANCE_COLS, USER_LOOKUP_ADVICE_COLS,
        USER_MAX_OUTPUTS, USER_MAX_SUBQUERIES, USER_RESULT_FIELD_ELEMENTS,
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
    let subq_aggr_params =         AggregationConfigParams {
        degree: K as u32,
        lookup_bits:LOOKUP_BITS,
        num_advice: USER_ADVICE_COLS,
        num_lookup_advice: USER_LOOKUP_ADVICE_COLS,
        num_fixed: USER_FIXED_COLS,
    };
    //  get_dummy_aggregation_params(K);
    //FROM https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/configs/test/subquery_aggregation_for_agg.json#L9
    let subq_aggr_break_points = vec![
        vec![
          1048565,
          1048566,
          1048566,
          1048566,
          1048564,
          1048565,
          1048566,
          1048565,
          1048566,
          1048565,
          1048564,
          1048566,
          1048564,
          1048566,
          1048565,
          1048564,
          1048566,
          1048566
        ]
      ];
    let subq_aggr_pinning = AggregationCircuitPinning::new(subq_aggr_params, subq_aggr_break_points);
    // kzg params for subq aggr circuit
    let kzg_params = gen_srs(K.try_into().unwrap());


    let base_params =         BaseCircuitParams {
        k: K,
        num_advice_per_phase: vec![USER_ADVICE_COLS],
        num_lookup_advice_per_phase: vec![USER_LOOKUP_ADVICE_COLS],
        num_fixed: USER_FIXED_COLS,
        lookup_bits: Some(LOOKUP_BITS),
        num_instance_columns: USER_INSTANCE_COLS,
    };
    let rlc_params = RlcCircuitParams { base: base_params, num_rlc_columns: NUM_RLC_COLUMNS };
    
    //OOOOORRRRR https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-eth/src/utils/eth_circuit.rs#L140
    // EthCircuitImpl::new(
    //     logic_inputs: I,
    //     prompt_rlc_params: RlcCircuitParams,
    //     promise_params: PromiseLoaderParams,
    // )
    
    //WIP
    // let rlc_thread_break_points = RlcThreadBreakPoints {}; //TODO
    // let rlc_circuit_pinning = RlcCircuitPinning::new(rlc_params, rlc_thread_break_points);

    //TODO let snark_storage, snark_account = generate_snark();

    // let subq_aggr_circuit = InputSubqueryAggregation {
    //     snark_header: header_snark,        //NEEDED?
    //     snark_results_root: results_snark, //NEEDED?
    //     snark_account: None,               //TODO
    //     snark_storage: None,               //TODO
    //     snark_solidity_mapping: None,
    //     snark_tx: None,
    //     snark_receipt: None,
    //     promise_commit_keccak: keccak_commit, //TODO
    // }
    // .prover_circuit(subq_aggr_pinning, &kzg_params)
    // .expect("subquery aggregation circuit");

    //CircuitBuilderStage::Keygen or Prove

    // let enhanced_snark = EnhancedSnark::new( , None);
    // let input_merkle_aggr = InputMerkleAggregation::new(vec![enhanced_snark]);
    // let aggr_circuit = input_merkle_aggr.prover_circuit(pinning, kzg_params);

    //SUBQUERY AGGREVGATION TESTS
    //   https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/src/subquery_aggregation/tests.rs#L306

    // let snark = gen_snark_shplonk(&params, &pk, prover_circuit, Some(snark_path));
    // let k = 20u32;
    // let params = gen_srs(k);



    // †††††††††††✟✟✟✟✟✟✟✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞
    // †††††††††††✟✟✟✟✟✟✟✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞
    // †††††††††††✟✟✟✟✟✟✟✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞
    // †††††††††††✟✟✟✟✟✟✟✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞
    // †††††††††††✟✟✟✟✟✟✟✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞
    // †††††††††††✟✟✟✟✟✟✟✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞
    // WIP :: TRYING THIS NOW!! => axiom_eth::utils::snark_verifier::create_universal_aggregation_circuit()
    // †††††††††††✟✟✟✟✟✟✟✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞
    // †††††††††††✟✟✟✟✟✟✟✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞
    // †††††††††††✟✟✟✟✟✟✟✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞
    // †††††††††††✟✟✟✟✟✟✟✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞
    // †††††††††††✟✟✟✟✟✟✟✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞

    //OOOOORRRRRRR https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-eth/src/utils/snark_verifier.rs#L140C49-L146C2
    // axiom_eth::utils::snark_verifier::create_universal_aggregation_circuit(
    //     stage: CircuitBuilderStage,
    //     circuit_params: AggregationCircuitParams,
    //     kzg_params: &ParamsKZG<Bn256>,
    //     snarks: Vec<Snark>,
    //     agg_vkey_hash_indices: Vec<Option<usize>>,
    // ) -> (AggregationCircuit, Vec<Vec<AssignedValue<F>>>, AssignedValue<F>);

    //FROM https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/configs/test/subquery_aggregation_for_agg.json#L2
    let aggr_circuit_params = AggregationConfigParams {
        degree: K as u32,
        lookup_bits: LOOKUP_BITS,
        num_advice: NUM_ADVICE,
        num_lookup_advice: NUM_LOOKUP_ADVICE,
        num_fixed: NUM_FIXED,
    };
    //TODO use gen_snark_shplonk() to generate `Snark`s
    //COPY https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/src/subquery_aggregation/tests.rs#L137
    let snark_account = gen_snark_shplonk(&kzg_params, &pk, component_circuit, Some(snark_path));
    let snark_storage = gen_snark_shplonk(&kzg_params, &pk, component_circuit, Some(snark_path));
    let snarks = vec![snark_account, snark_storage];
    let aggr_circuit_etc = axiom_eth::utils::snark_verifier::create_universal_aggregation_circuit(
        CircuitBuilderStage::Prover,
            aggr_circuit_params,
            &kzg_params,
            snarks,
            snarks.into_iter().map(|_| None).collect(),
        );
    //TODO do sth with aggr circuit

    
    //???????? SOME QUESTIONS
    // - how to choose params for BaseCircuitParams and AggregationConfigParams?
    // - for a simple storage proof we only need the storage shard and account shard circuit (no results_root_snark, no header_snark), correct?
    // - does 1 level of aggregation suffice to get an EVM verifier?


    //..... gen_evm_calldata_shplonk()
    //OOOOOOORRREND
}

// https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/src/subquery_aggregation/circuit.rs

// https://github.com/search?q=repo%3Aaxiom-crypto%2Faxiom-eth%20prover_circuit&type=code
// let component_circuit = load_prover_circuit(pinning);
// let mut prover_circuit = input.build(CircuitBuilderStage::Prover, pinning.params, &params)?;
// prover_circuit.set_break_points(pinning.break_points);
// let snark = gen_snark_shplonk(&params, &pk, prover_circuit, None::<&str>);
