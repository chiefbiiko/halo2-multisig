use axiom_codec::types::field_elements::AnySubqueryResult;
use axiom_eth::{
    halo2_base::AssignedValue,
    utils::component::{
        types::{FixLenLogical, Flatten},
        utils::get_logical_value,
        ComponentPromiseResultsInMerkle, ComponentType, FlattenVirtualTable, LogicalResult,
    },
};
// use itertools::Itertools;
// use serde::{Deserialize, Serialize};

// use crate::Field;
use axiom_query::Field;

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