//    Copyright (C) 2020  Kabir Kwatra
//    Full notice in main.rs file

use std::env;

pub fn wayne_id() -> u64 {
    env::var("WAYNE_ID")
        .expect("Missing WAYNE_ID in Env")
        .parse::<u64>()
        .expect("WAYNE_ID parsing failed")
}
