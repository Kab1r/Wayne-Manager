//    Copyright (C) 2020  Kabir Kwatra
//    Full notice in main.rs file

use std::env;
use std::sync::RwLock;

/// Returns Wayne's ID stored as an environment variable.
pub fn wayne_id() -> u64 {
    env::var("WAYNE_ID")
        .expect("Missing WAYNE_ID in Env")
        .parse::<u64>()
        .expect("WAYNE_ID parsing failed")
}

/// State of the Wayne Manager
pub struct WayneState {
    // A Lock containing the real name of the channel that Wayne is in.
    pub changed_channel_name: RwLock<Option<String>>,
}
