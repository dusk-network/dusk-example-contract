use dusk_bls12_381::BlsScalar;
use poseidon252::sponge::sponge::sponge_hash;

pub extern "C" fn p_hash(scalar_1: *const [u8; 32], scalar2: *const [u8; 32]) -> *const [u8; 32] {
    let res = sponge_hash(&[BlsScalar::zero()]);
    let mut res_arr: [u8; 32] = [0u8; 32];
    res_arr.copy_from_slice(&res.to_bytes()[..]);
    &res_arr as *const [u8; 32]
}
