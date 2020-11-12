use canonical::Store;
use dusk_bls12_381::BlsScalar;

use crate::leaf::ContractLeaf;
use crate::Contract;

impl<S: Store> Contract<S> {
    pub fn get_leaf(&self, idx: u64) -> ContractLeaf {
        self.tree().get(idx as usize).unwrap().unwrap()
    }

    pub fn state(&self) -> BlsScalar {
        self.get_state()
    }
}
