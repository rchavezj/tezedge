// Copyright (c) SimpleStaking and Tezedge Contributors
// SPDX-License-Identifier: MIT

use getset::Getters;
use serde::{Deserialize, Serialize};

use tezos_encoding::{
    encoding::{Encoding, Field, HasEncoding},
    has_encoding,
    types::BigInt,
};

use crate::non_cached_data;

#[derive(Serialize, Deserialize, Debug, Clone, Getters)]
pub struct Counter {
    #[get = "pub"]
    counter: BigInt,
}

impl Counter {
    pub fn to_numeric_string(&self) -> String {
        self.counter.0.to_str_radix(10)
    }
}

non_cached_data!(Counter);
has_encoding!(Counter, COUNTER_ENCODING, {
        Encoding::Obj(vec![
            Field::new("counter", Encoding::Z)
        ])
});

#[derive(Serialize, Deserialize, Debug, Clone, Getters)]
pub struct Balance {
    #[get = "pub"]
    balance: BigInt,
}

impl Balance {
    pub fn to_numeric_string(&self) -> String {
        self.balance.0.to_str_radix(10)
    }
}

non_cached_data!(Balance);
has_encoding!(Balance, BALANCE_ENCODING, {
    Encoding::Obj(vec![
        Field::new("balance", Encoding::Mutez)
    ])
});
