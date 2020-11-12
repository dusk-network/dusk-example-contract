use crate::leaf::ContractLeaf;
use crate::{ops, Contract};
use anyhow::{anyhow, Result};
use canonical_host::{MemStore, Transaction};
use dusk_bls12_381::BlsScalar;

type TransactionIndex = u16;

impl Contract<MemStore> {
    pub fn add_leaf(leaf: ContractLeaf) -> Transaction<(TransactionIndex, ContractLeaf), u64> {
        Transaction::new((ops::PUSH_LEAF, leaf))
    }
}
