use super::Host;
use crate::leaf::ContractLeaf;
use crate::{ops, Contract};
use anyhow::{anyhow, Result};
use canonical_host::{MemStore, Query};
use dusk_bls12_381::BlsScalar;

impl Contract<MemStore> {
    pub fn read_value_squared(
        host: &Host,
        pos: usize,
    ) -> Result<Query<(u8, ContractLeaf), BlsScalar>> {
        host.tree()
            .get(pos)
            .and_then(|leaf| leaf.ok_or(anyhow!("Leaf not found!")))
            .map(|leaf| Query::new((ops::QUERY_READ_VALUE_SQUARED, leaf)))
    }

    pub fn state() -> Query<u8, BlsScalar> {
        Query::new(ops::QUERY_STATE)
    }
}
