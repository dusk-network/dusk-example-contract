use canonical::Canon;
use canonical_derive::Canon;
use core::borrow::Borrow;
use dusk_bls12_381::Scalar;

#[derive(Debug, Default, Clone, Copy, Canon)]
pub struct ContractLeaf {
    pub s: Scalar,
    pos: u64,
}

impl Borrow<u64> for ContractLeaf {
    fn borrow(&self) -> &u64 {
        &self.pos
    }
}

impl From<u64> for ContractLeaf {
    fn from(n: u64) -> Self {
        Self {
            s: Scalar::from(n),
            pos: n,
        }
    }
}

#[cfg(feature = "host")]
mod host {
    use super::ContractLeaf;
    use canonical::Store;
    use dusk_bls12_381::Scalar;
    use poseidon252::sponge::sponge;
    use poseidon252::tree::PoseidonLeaf;

    impl<S: Store> PoseidonLeaf<S> for ContractLeaf {
        fn poseidon_hash(&self) -> Scalar {
            let pos = Scalar::from(self.pos);

            sponge::sponge_hash(&[self.s, pos])
        }

        fn pos(&self) -> u64 {
            self.pos
        }

        fn set_pos(&mut self, pos: u64) {
            self.pos = pos;
        }
    }
}
