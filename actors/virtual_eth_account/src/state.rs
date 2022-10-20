// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use fvm_ipld_encoding::tuple::*;
use fvm_ipld_encoding::Cbor;
use fvm_shared::address::Address;

/// State includes the address for the actor
#[derive(Serialize_tuple, Deserialize_tuple, Debug)]
pub struct State {
    // ID Address that is managing/pupetting this virtual account
    pub manager: Address,
}

impl Cbor for State {}