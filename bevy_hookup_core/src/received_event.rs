use bevy::prelude::*;

use crate::hook_session::SessionId;

#[derive(Event, Debug, Deref, DerefMut)]
pub struct ReceivedEvent<T> {
    #[deref]
    pub event: T,
    pub from_session: SessionId,
}
