use dusk_bls12_381::BlsScalar;
use canonical::Store;

use crate::leaf::ContractLeaf;
use crate::Contract;

impl<S: Store> Contract<S> {
    pub fn read_value_squared(&self, leaf: ContractLeaf) -> BlsScalar {
        leaf.s.square()
    }

    pub fn state(&self) -> BlsScalar {
        self.state
    }
}
