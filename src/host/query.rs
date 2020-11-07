use super::Host;
use crate::leaf::ContractLeaf;
use crate::Contract;
use anyhow::{anyhow, Result};
use canonical_host::Query;
use dusk_bls12_381::Scalar;

impl Contract {
    pub fn read_value_squared(
        host: &Host,
        pos: usize,
    ) -> Result<Query<(u16, ContractLeaf), Scalar>> {
        host.tree()
            .get(pos)
            .and_then(|leaf| leaf.ok_or(anyhow!("Leaf not found!")))
            .map(|leaf| Query::new((0, leaf)))
    }

    pub fn state() -> Query<u16, Scalar> {
        Query::new(1)
    }
}
