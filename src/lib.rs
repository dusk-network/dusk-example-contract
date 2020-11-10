// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

#![cfg_attr(feature = "hosted", no_std)]
#![feature(min_const_generics)]

use canonical::{Canon, Sink, Source, Store};
use dusk_bls12_381::BlsScalar;
use leaf::ContractLeaf;
use poseidon252::tree::{PoseidonMaxAnnotation, PoseidonTree};

pub mod ops {
    pub const QUERY_READ_VALUE_SQUARED: u8 = 0x00;
    pub const QUERY_STATE: u8 = 0x01;

    pub const TRANSACTION_SET_STATE_NEG: u8 = 0x00;
}

#[derive(Debug, Clone)]
pub struct Contract<S: Store> {
    state: BlsScalar,
    tree: PoseidonTree<ContractLeaf, PoseidonMaxAnnotation, S, 17>,
}

impl<S> Canon<S> for Contract<S>
where
    S: Store,
{
    fn read(source: &mut impl Source<S>) -> Result<Self, S::Error> {
        Ok(Contract {
            state: Canon::<S>::read(source)?,
            tree: Canon::<S>::read(source)?,
        })
    }

    fn write(&self, sink: &mut impl Sink<S>) -> Result<(), S::Error> {
        self.state.write(sink)?;
        self.tree.write(sink)
    }

    fn encoded_len(&self) -> usize {
        Canon::<S>::encoded_len(&self.state) + Canon::<S>::encoded_len(&self.tree)
    }
}

impl<S: Store> Contract<S> {
    pub fn new() -> Self {
        Self {
            state: BlsScalar::zero(),
            tree: PoseidonTree::new(),
        }
    }
}

pub mod leaf;

#[cfg(feature = "host")]
pub mod host;

#[cfg(feature = "hosted")]
pub mod hosted;
