// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

#![cfg_attr(feature = "hosted", no_std)]

use canonical::Canon;
use canonical_derive::Canon;
use dusk_bls12_381::Scalar;

pub mod ops {
    pub const QUERY_READ_VALUE_SQUARED: u8 = 0x00;
    pub const QUERY_STATE: u8 = 0x01;

    pub const TRANSACTION_SET_STATE_NEG: u8 = 0x00;
}

#[derive(Debug, Clone, Canon)]
pub struct Contract {
    state: Scalar,
}

impl Contract {
    pub fn new() -> Self {
        Self {
            state: Scalar::zero(),
        }
    }
}

pub mod leaf;

#[cfg(feature = "host")]
pub mod host;

#[cfg(feature = "hosted")]
pub mod hosted;
