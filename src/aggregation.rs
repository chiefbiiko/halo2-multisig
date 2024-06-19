use std::{
    collections::HashMap, fs::File, io::{Read, Write}, marker::PhantomData
};

use crate::{circuit::ComponentCircuitStorageSubquery, constants::*, subquery_aggregation::InputSubqueryAggregation, utils::test_fixture};
use axiom_eth::{
    halo2_base::{
        halo2_proofs::plonk,
        gates::circuit::{BaseCircuitParams, CircuitBuilderStage}, halo2_proofs::{
            halo2curves::bn256::{Bn256, Fr},
            poly::kzg::commitment::ParamsKZG,
        }, utils::fs::gen_srs
    }, rlc::circuit::RlcCircuitParams, snark_verifier_sdk::{halo2::{aggregation::AggregationConfigParams, gen_snark_shplonk}, CircuitExt}, utils::{
        build_utils::pinning::{
            aggregation::AggregationCircuitPinning, CircuitPinningInstructions, Halo2CircuitPinning, PinnableCircuit, RlcCircuitPinning
        }, component::promise_loader::single::PromiseLoaderParams, merkle_aggregation::InputMerkleAggregation, snark_verifier::{
            get_accumulator_indices, AggregationCircuitParams, EnhancedSnark,
            NUM_FE_ACCUMULATOR,
        }
    }
};

use axiom_codec::constants::{
        USER_ADVICE_COLS, USER_FIXED_COLS, USER_INSTANCE_COLS, USER_LOOKUP_ADVICE_COLS,
        USER_MAX_OUTPUTS, USER_MAX_SUBQUERIES, USER_RESULT_FIELD_ELEMENTS,
    };
use axiom_query::{components::subqueries::storage::types::{CircuitInputStorageShard, CircuitInputStorageSubquery}, keygen::shard::{ShardIntentAccount, ShardIntentStorage}};
use axiom_query::components::subqueries::storage::circuit::CoreParamsStorageSubquery;
use axiom_eth::halo2_base::utils::halo2::KeygenCircuitIntent;
use axiom_eth::utils::component::ComponentCircuit;

#[tokio::main]
async fn main() {
    // //TODO pining for subq aggr circuit
    // // type CircuitParams = AggregationConfigParams;
    // // type BreakPoints = MultiPhaseThreadBreakPoints;
    // let subq_aggr_params =         AggregationConfigParams {
    //     degree: K as u32,
    //     lookup_bits:LOOKUP_BITS,
    //     num_advice: USER_ADVICE_COLS,
    //     num_lookup_advice: USER_LOOKUP_ADVICE_COLS,
    //     num_fixed: USER_FIXED_COLS,
    // };
    // //  get_dummy_aggregation_params(K);
    // //FROM https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/configs/test/subquery_aggregation_for_agg.json#L9
    // let subq_aggr_break_points = vec![
    //     vec![
    //       1048565,
    //       1048566,
    //       1048566,
    //       1048566,
    //       1048564,
    //       1048565,
    //       1048566,
    //       1048565,
    //       1048566,
    //       1048565,
    //       1048564,
    //       1048566,
    //       1048564,
    //       1048566,
    //       1048565,
    //       1048564,
    //       1048566,
    //       1048566
    //     ]
    //   ];
    // let subq_aggr_pinning = AggregationCircuitPinning::new(subq_aggr_params, subq_aggr_break_points);
    // // kzg params for subq aggr circuit
    // let kzg_params = gen_srs(K.try_into().unwrap());


    // let base_params =         BaseCircuitParams {
    //     k: K,
    //     num_advice_per_phase: vec![USER_ADVICE_COLS],
    //     num_lookup_advice_per_phase: vec![USER_LOOKUP_ADVICE_COLS],
    //     num_fixed: USER_FIXED_COLS,
    //     lookup_bits: Some(LOOKUP_BITS),
    //     num_instance_columns: USER_INSTANCE_COLS,
    // };
    // let rlc_params = RlcCircuitParams { base: base_params, num_rlc_columns: NUM_RLC_COLUMNS };
    
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
    // WIP => axiom_eth::utils::snark_verifier::create_universal_aggregation_circuit()
    // †††††††††††✟✟✟✟✟✟✟✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞
    // †††††††††††✟✟✟✟✟✟✟✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞
    // †††††††††††✟✟✟✟✟✟✟✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞
    // †††††††††††✟✟✟✟✟✟✟✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞
    // †††††††††††✟✟✟✟✟✟✟✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞

    //WIP https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-eth/src/utils/snark_verifier.rs#L140C49-L146C2
    // axiom_eth::utils::snark_verifier::create_universal_aggregation_circuit(
    //     stage: CircuitBuilderStage,
    //     circuit_params: AggregationCircuitParams,
    //     kzg_params: &ParamsKZG<Bn256>,
    //     snarks: Vec<Snark>,
    //     agg_vkey_hash_indices: Vec<Option<usize>>,
    // ) -> (AggregationCircuit, Vec<Vec<AssignedValue<F>>>, AssignedValue<F>);

    let cargo_manifest_dir = env!("CARGO_MANIFEST_DIR");
    let storage_pinning_path = format!("{cargo_manifest_dir}/artifacts/storage_circuit_pinning.json");
    let storage_pk_path = format!("{cargo_manifest_dir}/artifacts/storage_circuit.pk");
    let storage_vk_path = format!("{cargo_manifest_dir}/artifacts/storage_circuit.vk");
    let storage_circuit_path = format!("{cargo_manifest_dir}/artifacts/storage_circuit.shplonk")
    let kzg_params = gen_srs(K.try_into().unwrap());

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

    let (storage_pk, storage_pinning, storage_circuit) = {
        let core_params = CoreParamsStorageSubquery {
            capacity: STORAGE_CAPACITY,
            max_trie_depth: STORAGE_PROOF_MAX_DEPTH,
        };
        let loader_params = (
            PromiseLoaderParams::new_for_one_shard(KECCAK_F_CAPACITY),
            PromiseLoaderParams::new_for_one_shard(ACCOUNT_CAPACITY),
        );
        let storage_intent = ShardIntentStorage {
            core_params,
            loader_params,
            k: K as u32,
            lookup_bits: LOOKUP_BITS,
        };
        let keygen_circuit = storage_intent.build_keygen_circuit();
        let (pk, pinning) = keygen_circuit.create_pk(&kzg_params, storage_pk_path, storage_pinning_path).expect("strg pk and pinning");
        let vk = pk.get_vk();
        let mut vk_file = File::create(storage_vk_path).expect("strg vk bin file");
        vk.write(&mut vk_file, axiom_eth::halo2_proofs::SerdeFormat::RawBytes).expect("strg vk bin write");

        //FROM https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/src/subquery_aggregation/tests.rs#L138
        let circuit = ComponentCircuitStorageSubquery::<Fr>::prover(
            core_params,
            loader_params,
            pinning,
        );

        (pk, pinning, circuit)
    };
   
    let (account_component_pk, account_component_circuit) = {
        //TODO create keygen account component circuit
        // let pk = TODO;
        // &|pinning| {
        //     let circuit = ComponentCircuitHeaderSubquery::<Fr>::prover(
        //         header_core_params.clone(),
        //         header_promise_params.clone(),
        //         pinning,
        //     );
        //     circuit.feed_input(Box::new(header_input.clone())).unwrap();
        //     circuit.fulfill_promise_results(&promise_results).unwrap();
        //     circuit
        // }
        //     (pk, circuit)
    };
    let snark_account = gen_snark_shplonk(&kzg_params, &pk, component_circuit, Some(snark_path));
    let snark_storage = gen_snark_shplonk(&kzg_params, &storage_pk, storage_circuit, Some(storage_circuit_path));
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
    // - is create_universal_aggregation_circuit() correct for aggregating component circuits?


    //..... gen_evm_calldata_shplonk()
    //WIPEND
}

//=====NOTES=====

//AXIOM PROD SUBQ AGGR https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/src/subquery_aggregation/circuit.rs
//RELATED              https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/src/subquery_aggregation/tests.rs#L150