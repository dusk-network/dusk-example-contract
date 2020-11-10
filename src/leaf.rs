use canonical::{Canon, Store};
use canonical_derive::Canon;
use core::borrow::Borrow;
use dusk_bls12_381::BlsScalar;

#[derive(Debug, Default, Clone, Copy, Canon)]
pub struct ContractLeaf {
    pub s: BlsScalar,
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
            s: BlsScalar::from(n),
            pos: n,
        }
    }
}

use poseidon252::tree::PoseidonLeaf;
#[cfg(feature = "hosted")]
impl<S> PoseidonLeaf<S> for ContractLeaf
where
    S: Store,
{
    fn poseidon_hash(&self) -> BlsScalar {
        unimplemented!();
    }

    fn pos(&self) -> u64 {
        self.pos
    }

    fn set_pos(&mut self, pos: u64) {
        self.pos = pos;
    }
}
#[cfg(feature = "host")]
use poseidon252::sponge::sponge::sponge_hash;
#[cfg(feature = "host")]
impl<S> PoseidonLeaf<S> for ContractLeaf
where
    S: Store,
{
    fn poseidon_hash(&self) -> BlsScalar {
        let pos = BlsScalar::from(self.pos);
        sponge_hash(&[self.s, pos])
    }

    fn pos(&self) -> u64 {
        self.pos
    }

    fn set_pos(&mut self, pos: u64) {
        self.pos = pos;
    }
}
