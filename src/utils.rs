use axiom_codec::types::field_elements::AnySubqueryResult;
use axiom_eth::{
    halo2_base::AssignedValue,
    utils::{
        build_utils::dummy::DummyFrom,
        component::{
            types::{FixLenLogical, Flatten},
            utils::get_logical_value,
            ComponentType, FlattenVirtualTable, LogicalResult,
        }
    },
    providers::storage::json_to_mpt_input,
};
// use itertools::Itertools;
// use serde::{Deserialize, Serialize};

use axiom_query::components::subqueries::account::types::GENESIS_ADDRESS_0_ACCOUNT_PROOF;

// use crate::Field;
use axiom_query::Field;

use core::marker::PhantomData;
use axiom_query::components::subqueries::storage::types::{CircuitInputStorageShard, CircuitInputStorageSubquery};
use ethers_core::types::EIP1186ProofResponse;
use super::CoreParamsStorageSubquery;

impl<F: Field> DummyFrom<CoreParamsStorageSubquery> for CircuitInputStorageShard<F> {
    fn dummy_from(core_params: CoreParamsStorageSubquery) -> Self {
        let CoreParamsStorageSubquery { capacity, max_trie_depth } = core_params;
        let request = {
            let pf: EIP1186ProofResponse =
                serde_json::from_str(GENESIS_ADDRESS_0_ACCOUNT_PROOF).unwrap();
            let proof = json_to_mpt_input(pf, 0, max_trie_depth);
            CircuitInputStorageSubquery { block_number: 0, proof }
        };
        Self { requests: vec![request; capacity], _phantom: PhantomData }
    }
}

pub(crate) fn extract_virtual_table<
    F: Field,
    S: Into<Flatten<AssignedValue<F>>>,
    T: Into<Flatten<AssignedValue<F>>>,
>(
    outputs: impl Iterator<Item = AnySubqueryResult<S, T>>,
) -> FlattenVirtualTable<AssignedValue<F>> {
    outputs.map(|output| (output.subquery.into(), output.value.into())).collect()
}

pub(crate) fn extract_logical_results<
    F: Field,
    S: FixLenLogical<AssignedValue<F>>,
    FS: FixLenLogical<F>,
    T: ComponentType<F, InputValue = FS, InputWitness = S, LogicalInput = FS>,
>(
    outputs: impl Iterator<Item = AnySubqueryResult<S, T::OutputWitness>>,
) -> Vec<LogicalResult<F, T>> {
    outputs
        .map(|output| {
            LogicalResult::<F, T>::new(
                get_logical_value(&output.subquery),
                get_logical_value(&output.value),
            )
        })
        .collect()
}