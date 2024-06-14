use axiom_eth::utils::{  
    snark_verifier::{
        get_accumulator_indices, AggregationCircuitParams, EnhancedSnark, NUM_FE_ACCUMULATOR,
    },
       merkle_aggregation::InputMerkleAggregation};



fn main() {
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