use std::sync::Mutex;

use serenity::model::id::ChannelId;
use serenity::model::voice::VoiceState;
use serenity::prelude::Context;

use lazy_static::lazy_static;

// Global State
lazy_static! {
    static ref CHANNEL_NAME: Mutex<String> = Mutex::new(String::new());
}
pub fn on_join(ctx: &Context, new_state: &VoiceState) {
    let channel_id = new_state
        .channel_id
        .expect("Unexpected None New State Channel Id");

    *CHANNEL_NAME.lock().unwrap() = channel_id.name(&ctx).expect("Unexpected None Channel Name");

    rename_voice_channel(ctx, channel_id, "Wayne".to_string());
}
pub fn on_leave(ctx: &Context, new_state: &VoiceState) {
    let channel_id = new_state
        .channel_id
        .expect("Unexpected None New State Channel Id");

    rename_voice_channel(ctx, channel_id, CHANNEL_NAME.lock().unwrap().clone());

    *CHANNEL_NAME.lock().unwrap() = String::new()
}

fn rename_voice_channel(ctx: &Context, channel_id: ChannelId, to: String) {
    channel_id.edit(ctx, |c| c.name(to)).unwrap();
}
