use crate::leaf::ContractLeaf;
use crate::Contract;
use canonical::Store;
use dusk_bls12_381::BlsScalar;

impl<S> Contract<S>
where
    S: Store,
{
    pub fn read_value_squared(&self, leaf: ContractLeaf) -> BlsScalar {
        leaf.s.square()
    }

    pub fn state(&self) -> BlsScalar {
        self.state
    }
}
