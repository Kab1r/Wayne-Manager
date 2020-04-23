use serenity::model::voice::VoiceState;

pub enum Event {
    WayneJoin,
    WayneLeave,
    WayneMove,
    NonWayne,
}

pub fn event(old: Option<VoiceState>, new_state: &VoiceState, wayne_id: &u64) -> Event {
    if new_state.user_id.as_u64() == wayne_id {
        if old.is_none() {
            return Event::WayneJoin;
        }
        let old_state = old.expect("Unexpected None Old Voice State");
        if new_state.channel_id.is_none() {
            return Event::WayneLeave;
        }
        if old_state
            .channel_id
            .expect("Unexpected None Old State Channel ID")
            != new_state
                .channel_id
                .expect("Unexpected None New State Channel ID")
        {
            return Event::WayneMove;
        }
    }
    return Event::NonWayne;
}
