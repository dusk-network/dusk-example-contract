use crate::leaf::ContractLeaf;
use crate::Contract;
use dusk_bls12_381::Scalar;

impl Contract {
    pub fn read_value_squared(&self, leaf: ContractLeaf) -> Scalar {
        leaf.s.square()
    }

    pub fn state(&self) -> Scalar {
        self.state
    }
}
