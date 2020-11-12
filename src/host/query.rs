use crate::leaf::ContractLeaf;
use crate::{ops, Contract};
use anyhow::{anyhow, Result};
use canonical_host::{MemStore, Query};
use dusk_bls12_381::BlsScalar;

type QueryIndex = u16;

impl Contract<MemStore> {
    pub fn get_leaf(pos: usize) -> Query<(QueryIndex, u64), ContractLeaf> {
        Query::new((ops::GET_LEAF, pos as u64))
    }

    pub fn state() -> Query<QueryIndex, BlsScalar> {
        Query::new(ops::QUERY_STATE)
    }
}
