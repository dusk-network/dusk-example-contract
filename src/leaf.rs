use canonical::{Canon, Store};
use canonical_derive::Canon;
use cfg_if::cfg_if;
use core::borrow::Borrow;
use core::ptr;
use dusk_bls12_381::BlsScalar;

#[derive(Debug, Default, Clone, Copy, Canon)]
pub struct ContractLeaf {
    pub s: BlsScalar,
    pub pos: u64,
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
    fn p_hash(ofs: &u8, len: u32, ret_addr: &mut [u8; 32]);
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
                    let mut s1 = [0u8;64];
                    s1[0..32].copy_from_slice(&inp[0].to_bytes()[..]);
                    s1[32..64].copy_from_slice(&inp[1].to_bytes()[..]);
                    let mut result_ffi = [0u8; 32];
                    p_hash(&s1[0], 64, &mut result_ffi);
                    result = BlsScalar::from_bytes(&result_ffi).unwrap()
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
