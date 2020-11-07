use super::Host;
use crate::leaf::ContractLeaf;
use crate::{ops, Contract};
use anyhow::{anyhow, Result};
use canonical_host::Query;
use dusk_bls12_381::Scalar;

impl Contract {
    pub fn read_value_squared(
        host: &Host,
        pos: usize,
    ) -> Result<Query<(u8, ContractLeaf), Scalar>> {
        host.tree()
            .get(pos)
            .and_then(|leaf| leaf.ok_or(anyhow!("Leaf not found!")))
            .map(|leaf| Query::new((ops::QUERY_READ_VALUE_SQUARED, leaf)))
    }

    pub fn state() -> Query<u8, Scalar> {
        Query::new(ops::QUERY_STATE)
    }
}
