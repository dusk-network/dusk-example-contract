use core::ptr;
use dusk_bls12_381::BlsScalar;
use poseidon252::sponge::sponge::sponge_hash;

pub unsafe extern "C" fn p_hash(
    scalar_1: *const [u8; 32],
    scalar2: *const [u8; 32],
    res: *mut [u8; 32],
) {
    let res_hash = sponge_hash(&[BlsScalar::zero()]);
    // Reuse the res pointer to avoid the array declaration
    let mut res_arr: [u8; 32] = [0u8; 32];
    res_arr.copy_from_slice(&res_hash.to_bytes()[..]);
    ptr::copy_nonoverlapping(&res_arr as *const [u8; 32], res, 32);
}
