//    Copyright (C) 2020  Kabir Kwatra
//    Full notice in main.rs file

use serenity::model::id::ChannelId;
use serenity::model::voice::VoiceState;
use serenity::prelude::Context;

use std::sync::RwLock;
use std::sync::RwLockWriteGuard;

/// Renames the channel to Wayne.
///
/// # Arguments
///
/// * `ctx` - The current Serenity context.
/// * `new_state` - The state of the voice channel to be renamed.
pub fn on_join(
    ctx: &Context,
    new_state: &VoiceState,
    channel_name_lock: &RwLock<Option<String>>,
) -> () {
    let channel_id = new_state
        .channel_id
        .expect("Unexpected None New State Channel Id");

    let mut global = unwrap_or_recover(channel_name_lock);
    *global = Some(channel_id.name(&ctx).expect("Unexpected None Channel Name"));

    rename_voice_channel(ctx, channel_id, "Wayne".to_string());
}

/// Renames the channel to what it was before Wayne joined.
///
/// # Arguments
///
/// * `ctx` - The current Serenity context.
/// * `new_state` - The state of the voice channel to be renamed.
pub fn on_leave(
    ctx: &Context,
    old_state: &VoiceState,
    channel_name_lock: &RwLock<Option<String>>,
) -> () {
    let channel_id = old_state
        .channel_id
        .expect("Unexpected None New State Channel Id");

    let mut global = unwrap_or_recover(channel_name_lock);
    rename_voice_channel(
        ctx,
        channel_id,
        global.clone().expect("Unexpected None global"),
    );
    *global = None;
}

/// Unwraps an RwLock for writing and recovers it if poisoned.
///
/// # Arguments
///
/// * `lock` - The RwLock to unwrap
fn unwrap_or_recover<T>(lock: &RwLock<T>) -> RwLockWriteGuard<T> {
    match lock.write() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

/// Renames the channel to a provided name.
///
/// # Arguments
///
/// * `ctx` - The current Serenity context.
/// * `channel_id` - The id of the voice channel to be renamed.
/// * `to` - The new name for the voice channel.
fn rename_voice_channel(ctx: &Context, channel_id: ChannelId, to: String) -> () {
    match channel_id.edit(ctx, |c| c.name(to)) {
        Ok(_) => (),
        Err(e) => println!("Channel Edit Error: {}", e),
    };
}
