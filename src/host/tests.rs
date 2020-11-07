use super::Host;
use crate::leaf::ContractLeaf;
use crate::Contract;
use canonical_host::{MemStore, Remote, Wasm};
use dusk_bls12_381::Scalar;

const BYTECODE: &'static [u8] =
    include_bytes!("../../target/wasm32-unknown-unknown/release/dusk_example_contract.wasm");

const INIT_LEAVES: usize = 257;

fn init() -> (Host, Remote<MemStore>) {
    let mut host = Host::default();
    for i in 0..INIT_LEAVES {
        host.tree_mut().push(ContractLeaf::from(i as u64)).unwrap();
    }

    let wasm = Wasm::new(Contract::new(), BYTECODE);
    let remote =
        Remote::new(wasm.clone(), host.as_ref()).expect("Failed to create the remote instance!");

    (host, remote)
}

#[test]
fn query() {
    let pos = 124;
    let val = ContractLeaf::from(pos).s.square();
    let (host, remote) = init();

    let store = <Host as AsRef<MemStore>>::as_ref(&host).clone();
    let cast = remote.cast::<Wasm<Contract, MemStore>>().unwrap();
    let query = Contract::read_value_squared(&host, pos as usize).unwrap();
    let val_remote = cast.query(&query, store).unwrap();

    let store = <Host as AsRef<MemStore>>::as_ref(&host).clone();
    let val_wasm = cast.query(&query, store).unwrap();

    assert_eq!(val, val_remote);
    assert_eq!(val, val_wasm);
}

#[test]
fn transact() {
    let pos = 101;
    let val = ContractLeaf::from(pos as u64).s.neg();
    let (host, mut remote) = init();

    let store = <Host as AsRef<MemStore>>::as_ref(&host).clone();
    let cast = remote.cast::<Wasm<Contract, MemStore>>().unwrap();
    let query = Contract::state();
    let state = cast.query(&query, store).unwrap();
    assert_eq!(Scalar::zero(), state);

    let store = <Host as AsRef<MemStore>>::as_ref(&host).clone();
    let mut cast_mut = remote.cast_mut::<Wasm<Contract, MemStore>>().unwrap();
    let transact = Contract::set_state_hash(&host, pos as usize).unwrap();
    cast_mut.transact(&transact, store).unwrap();
    cast_mut.commit().unwrap();

    let store = <Host as AsRef<MemStore>>::as_ref(&host).clone();
    let cast = remote.cast::<Wasm<Contract, MemStore>>().unwrap();
    let query = Contract::state();
    let state = cast.query(&query, store).unwrap();
    assert_eq!(val, state);
}
