use std::{
    collections::HashMap, fs::File, io::{Read, Write}, marker::PhantomData
};

use crate::{circuit::ComponentCircuitStorageSubquery, constants::*, subquery_aggregation::InputSubqueryAggregation, utils::test_fixture};
use axiom_eth::{
    halo2_base::{
        gates::circuit::{BaseCircuitParams, CircuitBuilderStage}, halo2_proofs::{halo2curves::bn256::{Bn256, Fr}, plonk, poly::kzg::commitment::ParamsKZG}, utils::fs::gen_srs
    }, keccak::promise::generate_keccak_shards_from_calls, rlc::circuit::RlcCircuitParams, snark_verifier_sdk::{halo2::{aggregation::AggregationConfigParams, gen_snark_shplonk}, CircuitExt}, utils::{build_utils::pinning::{
            aggregation::AggregationCircuitPinning, CircuitPinningInstructions, Halo2CircuitPinning, PinnableCircuit, RlcCircuitPinning
        }, component::{promise_loader::{comp_loader::SingleComponentLoaderParams, multi::MultiPromiseLoaderParams, single::PromiseLoaderParams}, ComponentPromiseResultsInMerkle, ComponentType, SelectedDataShardsInMerkle}, merkle_aggregation::InputMerkleAggregation, snark_verifier::{get_accumulator_indices, AggregationCircuitParams, EnhancedSnark, NUM_FE_ACCUMULATOR}}
};

use axiom_codec::{constants::{
        NUM_SUBQUERY_TYPES, USER_ADVICE_COLS, USER_FIXED_COLS, USER_INSTANCE_COLS, USER_LOOKUP_ADVICE_COLS, USER_MAX_OUTPUTS, USER_MAX_SUBQUERIES, USER_RESULT_FIELD_ELEMENTS
    }, types::native::SubqueryType};
use axiom_query::{components::{results::circuit::{ComponentCircuitResultsRoot, CoreParamsResultRoot}, subqueries::{account::{circuit::{ComponentCircuitAccountSubquery, CoreParamsAccountSubquery}, types::ComponentTypeAccountSubquery}, block_header::{circuit::{ComponentCircuitHeaderSubquery, CoreParamsHeaderSubquery}, types::ComponentTypeHeaderSubquery}, storage::types::{CircuitInputStorageShard, CircuitInputStorageSubquery, ComponentTypeStorageSubquery}}}, keygen::shard::{ShardIntentAccount, ShardIntentHeader, ShardIntentStorage}};
use axiom_query::components::subqueries::storage::circuit::CoreParamsStorageSubquery;
use axiom_eth::halo2_base::utils::halo2::KeygenCircuitIntent;
use axiom_eth::utils::component::ComponentCircuit;

#[tokio::main]
async fn main() {
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
    // WIP => th/axiom-query/src/subquery_aggregation.rs ::prover_circuit()
    // †††††††††††✟✟✟✟✟✟✟✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞
    // †††††††††††✟✟✟✟✟✟✟✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞
    // †††††††††††✟✟✟✟✟✟✟✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞
    // †††††††††††✟✟✟✟✟✟✟✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞
    // †††††††††††✟✟✟✟✟✟✟✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✝✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞

    //WIP https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/src/subquery_aggregation/circuit.rs#L239

    let cargo_manifest_dir = env!("CARGO_MANIFEST_DIR");
    let storage_pinning_path = format!("{cargo_manifest_dir}/artifacts/storage_circuit_pinning.json");
    let storage_pk_path = format!("{cargo_manifest_dir}/artifacts/storage_circuit.pk");
    let storage_vk_path = format!("{cargo_manifest_dir}/artifacts/storage_circuit.vk");
    let storage_circuit_path = format!("{cargo_manifest_dir}/artifacts/storage_circuit.shplonk");
    let account_pinning_path = format!("{cargo_manifest_dir}/artifacts/account_circuit_pinning.json");
    let account_pk_path = format!("{cargo_manifest_dir}/artifacts/account_circuit.pk");
    let account_vk_path = format!("{cargo_manifest_dir}/artifacts/account_circuit.vk");
    let account_circuit_path = format!("{cargo_manifest_dir}/artifacts/account_circuit.shplonk");
    let header_pinning_path = format!("{cargo_manifest_dir}/artifacts/header_circuit_pinning.json");
    let header_pk_path = format!("{cargo_manifest_dir}/artifacts/header_circuit.pk");
    let header_vk_path = format!("{cargo_manifest_dir}/artifacts/header_circuit.vk");
    let header_circuit_path = format!("{cargo_manifest_dir}/artifacts/header_circuit.shplonk");
    std::env::set_var("PARAMS_DIR", format!("{cargo_manifest_dir}/artifacts"));
    let kzg_params = gen_srs(K.try_into().unwrap());

    //FROM https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/configs/test/subquery_aggregation_for_agg.json#L2
    let aggr_circuit_params = AggregationConfigParams {
        degree: K as u32,
        lookup_bits: LOOKUP_BITS,
        num_advice: NUM_ADVICE,
        num_lookup_advice: NUM_LOOKUP_ADVICE,
        num_fixed: NUM_FIXED,
    };

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
        let (pk, pinning) = keygen_circuit.create_pk(&kzg_params, &storage_pk_path, &storage_pinning_path).expect("strg pk and pinning");
        let mut vk_file = File::create(&storage_vk_path).expect("strg vk bin file");
        pk.get_vk().write(&mut vk_file, axiom_eth::halo2_proofs::SerdeFormat::RawBytes).expect("strg vk bin write");
        //FROM https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/src/subquery_aggregation/tests.rs#L138
        let storage_circuit = ComponentCircuitStorageSubquery::<Fr>::prover(
            core_params,
            loader_params,
            pinning,
        );
        // TODO feed input to storage shard - only to storage shard bc it is our entry!?
        // storage_circuit.feed_input(Box::new(input)).unwrap(); whyhow, still probly feed input here??????
        (pk, pinning, storage_circuit)
    };
   
    let (account_pk, account_pinning, account_circuit) = {
        let core_params = CoreParamsAccountSubquery {
            capacity: ACCOUNT_CAPACITY,
            max_trie_depth: ACCOUNT_PROOF_MAX_DEPTH,
        };
        let loader_params = (
            PromiseLoaderParams::new_for_one_shard(KECCAK_F_CAPACITY),
            PromiseLoaderParams::new_for_one_shard(HEADER_CAPACITY),
        );
        let account_intent = ShardIntentAccount {
            core_params,
            loader_params,
            k: K as u32,
            lookup_bits: LOOKUP_BITS,
        };
        let keygen_circuit = account_intent.build_keygen_circuit();
        let (pk, pinning) = keygen_circuit.create_pk(&kzg_params, &account_pk_path, &account_pinning_path).expect("acnt pk and pinning");
        let mut vk_file = File::create(&account_vk_path).expect("acnt vk bin file");
        pk.get_vk().write(&mut vk_file, axiom_eth::halo2_proofs::SerdeFormat::RawBytes).expect("acnt vk bin write");
        //FROM https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/src/subquery_aggregation/tests.rs#L138
        let account_circuit = ComponentCircuitAccountSubquery::<Fr>::prover(
            core_params,
            loader_params,
            pinning,
        );
        //IGNORE for now - think we dont need to feed input to the account component
        // account_circuit.feed_input(Box::new(input)).unwrap(); why feed input here??????
        (pk, pinning, account_circuit)
    };

    let (header_pk, header_pinning, header_circuit) = {
        let core_params = CoreParamsHeaderSubquery {
            capacity: HEADER_CAPACITY,
            max_extra_data_bytes: MAX_EXTRA_DATA_BYTES,
        };
        let loader_params= PromiseLoaderParams::new_for_one_shard(KECCAK_F_CAPACITY);
        let header_intent = ShardIntentHeader {
            core_params,
            loader_params,
            k: K as u32,
            lookup_bits: LOOKUP_BITS,
        };
        let keygen_circuit = header_intent.build_keygen_circuit();
        let (pk, pinning) = keygen_circuit.create_pk(&kzg_params, &header_pk_path, &header_pinning_path).expect("hdr pk and pinning");
        let mut vk_file = File::create(&header_vk_path).expect("hdr vk bin file");
        pk.get_vk().write(&mut vk_file, axiom_eth::halo2_proofs::SerdeFormat::RawBytes).expect("hdr vk bin write");
        //FROM https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/src/subquery_aggregation/tests.rs#L138
        let header_circuit = ComponentCircuitHeaderSubquery::<Fr>::prover(
            core_params,
            loader_params,
            pinning,
        );
        //IGNORE for now - think we dont need to feed input to the header component
        // header_circuit.feed_input(Box::new(input)).unwrap(); why feed input here??????
        (pk, pinning, header_circuit)
    };


    let (results_pk, results_pinning, results_circuit) = {

        let mut enabled_types = [false; NUM_SUBQUERY_TYPES];
        enabled_types[SubqueryType::Storage as usize] = true;
        enabled_types[SubqueryType::Account as usize] = true;
        enabled_types[SubqueryType::Header as usize] = true;
        let mut params_per_comp = HashMap::new();
        params_per_comp.insert(
            ComponentTypeHeaderSubquery::<Fr>::get_type_id(),
            SingleComponentLoaderParams::new(0, vec![3]),
        );
        params_per_comp.insert(
            ComponentTypeAccountSubquery::<Fr>::get_type_id(),
            SingleComponentLoaderParams::new_for_one_shard(ACCOUNT_CAPACITY),
        );
        params_per_comp.insert(
            ComponentTypeStorageSubquery::<Fr>::get_type_id(),
            SingleComponentLoaderParams::new_for_one_shard(STORAGE_CAPACITY),
        );
        let promise_results_params = MultiPromiseLoaderParams { params_per_component: params_per_comp };
    
        //WIP CircuitInputResultsRootShard
        // SubqueryResultsTable::new(vec![]);
        //FlattenedSubqueryResult::new(SubqueryKey([T;csubqkeylen]),SubqueryOutput([T, cmaxsubqout]))
        // shard_into_component_promise_results()
        //==>>>>>> https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/src/components/subqueries/storage/tests.rs#L54
        let results_input = TODO;
        let mut results_circuit = ComponentCircuitResultsRoot::<Fr>::new(
            CoreParamsResultRoot { enabled_types, capacity: results_input.subqueries.len() },
            (PromiseLoaderParams::new_for_one_shard(200), promise_results_params.clone()),
            rlc_params,
        );
        results_circuit.feed_input(Box::new(results_input.clone())).expect("feed results");
        //WIP
        let promise_results = HashMap::new();
        results_circuit.fulfill_promise_results(&promise_results).unwrap();
        results_circuit.calculate_params();
    
        // let results_snark =
        //     generate_snark("results_root_for_agg", params, results_circuit, &|pinning| {
        //         let results_circuit = ComponentCircuitResultsRoot::<Fr>::prover(
        //             CoreParamsResultRoot { enabled_types, capacity: results_input.subqueries.len() },
        //             (PromiseLoaderParams::new_for_one_shard(200), promise_results_params.clone()),
        //             pinning,
        //         );
        //         results_circuit.feed_input(Box::new(results_input.clone())).unwrap();
        //         results_circuit.fulfill_promise_results(&promise_results).unwrap();
        //         results_circuit
        //     })?;

    };

    let snark_header = gen_snark_shplonk(&kzg_params, &header_pk, header_circuit, Some(&header_circuit_path));
    let snark_account = gen_snark_shplonk(&kzg_params, &account_pk, account_circuit, Some(&account_circuit_path));
    let snark_storage = gen_snark_shplonk(&kzg_params, &storage_pk, storage_circuit, Some(&storage_circuit_path));

    // get keccak calls originating from storage shard that got input //~?
    let output_keccak_shard = generate_keccak_shards_from_calls(&storage_circuit, KECCAK_F_CAPACITY).expect("keccak calls");
    let keccak_merkle = ComponentPromiseResultsInMerkle::<Fr>::from_single_shard(
        output_keccak_shard.into_logical_results(),
    );
    let keccak_commit = keccak_merkle.leaves()[0].commit;


    let subq_aggr_circuit = InputSubqueryAggregation {
        snark_header: EnhancedSnark{inner: snark_header, agg_vk_hash_idx:None},        // account needs header
        snark_results_root: results_snark, //TODO everything needs results root
        snark_account: Some(EnhancedSnark{inner: snark_account, agg_vk_hash_idx:None}), // account needs header
        snark_storage: Some(EnhancedSnark{inner: snark_storage, agg_vk_hash_idx:None}), // storage needs account         
        snark_solidity_mapping: None,
        snark_tx: None,
        snark_receipt: None,
        promise_commit_keccak: keccak_commit, //~?
    }
    .prover_circuit(subq_aggr_pinning, &kzg_params)
    .expect("subquery aggregation circuit");

    //TODO do sth with aggr circuit


    
    //???????? SOME QUESTIONS
    // - how to aggregate from component storage circuit to evm verifier circuit?
    //   ..in our scenario where we want to generate a single storage proof proof:
    //     - pass `CircuitInputStorageSubquery` to `storage_circuit.feed_input(Box::new(input))` above?
    //     - after `feed_input()` do we need to call `circuit.fulfill_promise_results(&promise_results)`?
    //     - if yes, how do we get these ---------------------------------------------/\/\/\/\/\/\/\/\  ?
    //   
    // - does 1 level of aggregation suffice to get an EVM verifier?
    //     -> no we need at least one more level of aggregation to verify keccak promise commitments
    //     -> see https://github.com/axiom-crypto/axiom-eth/tree/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query#subquery-aggregation-circuit
    // - 

    //WIPEND
}

//=====NOTES=====

//AXIOM PROD SUBQ AGGR https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/src/subquery_aggregation/circuit.rs
//RELATED              https://github.com/axiom-crypto/axiom-eth/blob/0a218a7a68c5243305f2cd514d72dae58d536eff/axiom-query/src/subquery_aggregation/tests.rs#L150
//... gen_evm_proof_shplonk()
//... gen_evm_calldata_shplonk()
//... gen_evm_verifier_shplonk::<AggregationCircuit>(