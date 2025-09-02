use bevy::prelude::*;

use crate::session::SessionChannels;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct SessionId(pub i32);

pub trait SessionMessager<TSendables> {
    fn component_added(
        &self,
        channels: &SessionChannels<TSendables>,
        entity: Entity,
        component_data: TSendables,
    );
    fn componend_updated(
        &self,
        channels: &SessionChannels<TSendables>,
        entity: Entity,
        component_data: TSendables,
    );
    fn component_removed(&self, channels: &SessionChannels<TSendables>, entity: Entity);
}
