use serenity::model::id::GuildId;
use serenity::model::voice::VoiceState;
use serenity::prelude::*;

mod wayne_data;
pub use wayne_data::WAYNE_ID;

mod event;
pub use event::{event, Event};

mod on_event;
pub use on_event::{on_join, on_leave};

pub struct Handler;
impl EventHandler for Handler {
    fn voice_state_update(
        &self,
        ctx: Context,
        _guild_id: Option<GuildId>,
        old: Option<VoiceState>,
        new_state: VoiceState,
    ) {
        match event(old, &new_state, &WAYNE_ID) {
            // On Join Event
            Event::WayneJoin => on_join(&ctx, &new_state),

            // On Leave Event
            Event::WayneLeave => on_leave(&ctx, &new_state),

            // On Move Event
            Event::WayneMove => {
                on_join(&ctx, &new_state);
                on_leave(&ctx, &new_state);
            }
            // On NoneWayne Event
            Event::NonWayne => {}
        }
    }
}
