use super::Host;
use crate::leaf::ContractLeaf;
use crate::{ops, Contract};
use anyhow::{anyhow, Result};
use canonical_host::{MemStore, Transaction};
use dusk_bls12_381::BlsScalar;

impl Contract<MemStore> {
    pub fn set_state_neg(
        host: &Host,
        pos: usize,
    ) -> Result<Transaction<(u8, ContractLeaf), BlsScalar>> {
        host.tree()
            .get(pos)
            .and_then(|leaf| leaf.ok_or(anyhow!("Leaf not found!")))
            .map(|leaf| Transaction::new((ops::PUSH_LEAF, leaf)))
    }
}
