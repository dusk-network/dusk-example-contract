use super::Host;
use crate::leaf::ContractLeaf;
use crate::Contract;
use anyhow::{anyhow, Result};
use canonical_host::Transaction;
use dusk_bls12_381::Scalar;

impl Contract {
    pub fn set_state_hash(
        host: &Host,
        pos: usize,
    ) -> Result<Transaction<(u16, ContractLeaf), Scalar>> {
        host.tree()
            .get(pos)
            .and_then(|leaf| leaf.ok_or(anyhow!("Leaf not found!")))
            .map(|leaf| Transaction::new((0, leaf)))
    }
}
