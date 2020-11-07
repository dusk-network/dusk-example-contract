use crate::leaf::ContractLeaf;
use crate::Contract;
use dusk_bls12_381::Scalar;

impl Contract {
    pub fn set_state_neg(&mut self, leaf: ContractLeaf) -> Scalar {
        self.state = leaf.s.neg();
        self.state
    }
}
