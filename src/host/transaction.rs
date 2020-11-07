use super::Host;
use crate::leaf::ContractLeaf;
use crate::{ops, Contract};
use anyhow::{anyhow, Result};
use canonical_host::Transaction;
use dusk_bls12_381::Scalar;

impl Contract {
    pub fn set_state_neg(
        host: &Host,
        pos: usize,
    ) -> Result<Transaction<(u8, ContractLeaf), Scalar>> {
        host.tree()
            .get(pos)
            .and_then(|leaf| leaf.ok_or(anyhow!("Leaf not found!")))
            .map(|leaf| Transaction::new((ops::TRANSACTION_SET_STATE_NEG, leaf)))
    }
}
