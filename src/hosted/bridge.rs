use crate::leaf::ContractLeaf;
use crate::{ops, Contract};
use canonical::{BridgeStore, ByteSink, ByteSource, Canon, Id32, Store};

const PAGE_SIZE: usize = 1024 * 4;

type BS = BridgeStore<Id32>;
type QueryIndex = u16;
type TransactionIndex = u16;

fn query(bytes: &mut [u8; PAGE_SIZE]) -> Result<(), <BS as Store>::Error> {
    let store = BS::default();
    let mut source = ByteSource::new(&bytes[..], store.clone());

    // read self.
    let slf: Contract<BS> = Canon::<BS>::read(&mut source)?;

    // read query id
    let qid: QueryIndex = Canon::<BS>::read(&mut source)?;
    match qid {
        // get_leaf (&Self, u64) -> ContractLeaf
        ops::GET_LEAF => {
            // Read idx
            let idx: u64 = Canon::<BS>::read(&mut source)?;
            // Get the leaf
            let ret = slf.get_leaf(idx);
            let mut sink = ByteSink::new(&mut bytes[..], store.clone());
            Canon::<BS>::write(&ret, &mut sink)?;
            Ok(())
        }
        // state (&Self) -> Scalar
        ops::QUERY_STATE => {
            // Get state
            let ret = slf.state();
            let mut sink = ByteSink::new(&mut bytes[..], store.clone());
            Canon::<BS>::write(&ret, &mut sink)?;
            Ok(())
        }
        _ => panic!(""),
    }
}

#[no_mangle]
fn q(bytes: &mut [u8; PAGE_SIZE]) {
    let _ = query(bytes);
}

fn transaction(bytes: &mut [u8; PAGE_SIZE]) -> Result<(), <BS as Store>::Error> {
    let store = BS::default();
    let mut source = ByteSource::new(bytes, store.clone());

    // read self.
    let mut slf: Contract<BS> = Canon::<BS>::read(&mut source)?;
    // read transaction id
    let qid: TransactionIndex = Canon::<BS>::read(&mut source)?;
    match qid {
        // push_leaf (&mut Self, leaf: ContractLeaf)
        ops::PUSH_LEAF => {
            // read multiple args
            let leaf: ContractLeaf = Canon::<BS>::read(&mut source)?;
            let idx = slf.push_leaf(leaf);
            let mut sink = ByteSink::new(&mut bytes[..], store.clone());
            // return new state
            Canon::<BS>::write(&slf, &mut sink)?;
            // return result
            Canon::<BS>::write(&(idx as u64), &mut sink)
        }
        _ => panic!(""),
    }
}

#[no_mangle]
fn t(bytes: &mut [u8; PAGE_SIZE]) {
    transaction(bytes).unwrap()
}
