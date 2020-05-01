//    Copyright (C) 2020  Kabir Kwatra
//    Full notice in main.rs file

use serenity::model::id::GuildId;
use serenity::model::voice::VoiceState;
use serenity::prelude::{Context, EventHandler};

mod wayne_data;
use wayne_data::wayne_id;
pub use wayne_data::WayneState;

mod event;
use event::{event, Event};

mod on_event;
use on_event::{on_join, on_leave};

/// Handles Discord Events.
pub struct Handler {
    // State of the Handler
    pub state: WayneState,
}
impl EventHandler for Handler {
    /// Dispatched when a user joins, leaves or moves to a voice channel.
    /// Provides the guild's id (if available) and the old and the new state of the guild's voice channels.
    fn voice_state_update(
        &self,
        ctx: Context,
        _: Option<GuildId>,
        old: Option<VoiceState>,
        new_state: VoiceState,
    ) -> () {
        match event(&old, &new_state, &wayne_id()) {
            // On Join Event
            Event::WayneJoin => on_join(&ctx, &new_state, &self.state.changed_channel_name),

            // On Leave Event
            Event::WayneLeave => on_leave(&ctx, &old.unwrap(), &self.state.changed_channel_name),

            // On Move Event
            Event::WayneMove => {
                on_leave(&ctx, &old.unwrap(), &self.state.changed_channel_name);
                on_join(&ctx, &new_state, &self.state.changed_channel_name);
            }
            // On NoneWayne Event (default)
            _ => (),
        }
    }
}
