use canonical::Store;
use dusk_bls12_381::BlsScalar;

use crate::leaf::ContractLeaf;
use crate::Contract;

impl<S: Store> Contract<S> {
    pub fn push_leaf(&mut self, leaf: ContractLeaf) -> usize {
        self.tree_mut().push(leaf).unwrap()
    }
}
