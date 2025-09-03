use bevy::prelude::*;

use crate::{external_component::ExternalComponent, session::SessionChannels, sync_id::SyncId};

#[derive(Reflect, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct SessionId(pub i32);

pub trait SessionMessenger<TSendables> {
    fn entity_added(&self, channels: &SessionChannels<TSendables>, sync_id: SyncId);
    fn entity_removed(&self, channels: &SessionChannels<TSendables>, sync_id: SyncId);
    fn component_added(
        &self,
        channels: &SessionChannels<TSendables>,
        external_component: ExternalComponent,
        component_data: TSendables,
    );
    fn componend_updated(
        &self,
        channels: &SessionChannels<TSendables>,
        external_component: ExternalComponent,
        component_data: TSendables,
    );
    fn component_removed(
        &self,
        channels: &SessionChannels<TSendables>,
        external_component: ExternalComponent,
    );
}
