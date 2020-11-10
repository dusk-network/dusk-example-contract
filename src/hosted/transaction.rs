use crate::leaf::ContractLeaf;
use crate::Contract;
use canonical::Store;
use dusk_bls12_381::BlsScalar;

impl Contract {
    pub fn set_state_neg(&mut self, leaf: ContractLeaf) -> BlsScalar {
        self.state = leaf.s.neg();
        self.state
    }
}
