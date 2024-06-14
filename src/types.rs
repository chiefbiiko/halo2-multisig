use axiom_eth::{
    halo2_base::safe_types::{SafeAddress, SafeBytes32},
    Field,
    {
        mpt::MPTProofWitness,
        rlp::types::{RlpArrayWitness, RlpFieldWitness},
        storage::circuit::EthStorageInput,
    },
};
use getset::Getters;
use serde::{Deserialize, Serialize};

/// Circuit input for a single Storage subquery.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CircuitInputStorageSubquery {
    /// The block number to access the storage state at.
    pub block_number: u64,
    /// Storage proof formatted as MPT input. It will contain the account address.
    /// ### Warning
    /// `proof.acct_pf` will be empty and `proof` will **not** have state_root set.
    pub proof: EthStorageInput,
}

/// Stores storage slot information as well as a proof of inclusion to be verified in parse_storage_phase1. Is returned
/// by `parse_storage_phase0`.
#[derive(Clone, Debug, Getters)]
pub struct EthStorageWitness<F: Field> {
    pub slot: SafeBytes32<F>,
    #[getset(get = "pub")]
    pub(crate) value_witness: RlpFieldWitness<F>,
    #[getset(get = "pub")]
    pub(crate) mpt_witness: MPTProofWitness<F>,
}

/// Stores Account information to be used in later functions. Is returned by `parse_account_proof_phase0`.
#[derive(Clone, Debug, Getters)]
pub struct EthAccountWitness<F: Field> {
    pub address: SafeAddress<F>,
    #[getset(get = "pub")]
    pub(crate) array_witness: RlpArrayWitness<F>,
    #[getset(get = "pub")]
    pub(crate) mpt_witness: MPTProofWitness<F>,
}
