use crate::leaf::ContractLeaf;
use crate::Contract;
use canonical_host::{MemStore, Remote, Wasm};
use dusk_bls12_381::BlsScalar;

const BYTECODE: &'static [u8] =
    include_bytes!("../../target/wasm32-unknown-unknown/release/dusk_example_contract.wasm");

#[test]
fn add_get_leaf() {
    // Init Env & Contract
    let store = MemStore::new();
    let wasm_contract = Wasm::new(Contract::new(), BYTECODE);
    let mut remote = Remote::new(wasm_contract, &store).unwrap();
    // Create ContractLeaf
    let leaf = ContractLeaf::from(0u64);
    // Add leaf to the Contract's tree and get it's pos index back
    let mut cast = remote
        .cast_mut::<Wasm<Contract<MemStore>, MemStore>>()
        .unwrap();
    cast.transact(&Contract::add_leaf(leaf), store.clone())
        .unwrap();
    cast.commit().unwrap();
    // If call succeeds, this should not fail.

    // Check wether the

    let pushed_leaf = remote
        .cast::<Wasm<Contract<MemStore>, MemStore>>()
        .unwrap()
        .query(&Contract::get_leaf(0), store.clone())
        .unwrap();

    assert_eq!(pushed_leaf.pos, 0u64);
    assert_eq!(pushed_leaf.s, BlsScalar::zero());
}

#[test]
fn get_state() {
    // Init Env & Contract
    let store = MemStore::new();
    let wasm_contract = Wasm::new(Contract::new(), BYTECODE);
    let mut remote = Remote::new(wasm_contract, &store).unwrap();

    // Check that initial state value is 0
    assert_eq!(
        remote
            .cast::<Wasm<Contract<MemStore>, MemStore>>()
            .unwrap()
            .query(&Contract::state(), store.clone())
            .unwrap(),
        BlsScalar::zero()
    );
}
