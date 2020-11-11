use canonical::{Canon, Store};
use canonical_derive::Canon;
use cfg_if::cfg_if;
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

extern "C" {
    fn p_hash(scalar_1: *const [u8; 32], scalar2: *const [u8; 32]) -> *const [u8; 32];
}

#[cfg(feature = "host")]
use poseidon252::sponge::sponge::sponge_hash;

use poseidon252::tree::PoseidonLeaf;
impl<S> PoseidonLeaf<S> for ContractLeaf
where
    S: Store,
{
    fn poseidon_hash(&self) -> BlsScalar {
        let inp = [self.s, BlsScalar::from(self.pos)];
        let mut result = BlsScalar::zero();
        cfg_if! {
            if #[cfg(feature = "host")] {
                result = sponge_hash(&inp);
            }
            else if #[cfg(feature = "hosted")] {
                unsafe {
                    let mut s1 = [0u8; 32];
                    s1.copy_from_slice(&inp[0].to_bytes()[..]);
                    let mut s2 = [0u8; 32];
                    s2.copy_from_slice(&inp[0].to_bytes()[..]);
                    let res_ffi = p_hash(&s1 as *const [u8; 32], &s2 as *const [u8; 32]);

                }
            }
        }
        result
    }

    fn pos(&self) -> u64 {
        self.pos
    }

    fn set_pos(&mut self, pos: u64) {
        self.pos = pos;
    }
}
