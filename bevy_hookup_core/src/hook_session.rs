use bevy::prelude::*;

use crate::{
    external_component::ExternalComponent,
    session::{Session, SessionChannels},
    sync_id::SyncId,
};

#[derive(Reflect, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct SessionId(u64);

impl Default for SessionId {
    fn default() -> Self {
        Self(rand::random())
    }
}

pub trait SessionMessenger<TSendables> {
    fn to_session(self) -> Session<TSendables>;
    fn get_session_id(&self) -> SessionId;
    fn get_channels(&self) -> SessionChannels<TSendables>;
    fn entity_added(&mut self, channels: &SessionChannels<TSendables>, sync_id: SyncId);
    fn entity_removed(&mut self, channels: &SessionChannels<TSendables>, sync_id: SyncId);
    fn component_added(
        &mut self,
        channels: &SessionChannels<TSendables>,
        external_component: ExternalComponent,
        component_data: TSendables,
    );
    fn componend_updated(
        &mut self,
        channels: &SessionChannels<TSendables>,
        external_component: ExternalComponent,
        component_data: TSendables,
    );
    fn component_removed(
        &mut self,
        channels: &SessionChannels<TSendables>,
        external_component: ExternalComponent,
    );
}
