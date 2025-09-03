use bevy::prelude::*;
use bevy_hookup_core::{
    external_component::ExternalComponent,
    hook_session::{SessionId, SessionMessenger},
    session::{AddedData, EntityActions, RemovedData, SessionChannels, UpdatedData},
    sync_id::SyncId,
};

pub struct SelfSession {
    session_id: SessionId,
}

impl SelfSession {
    pub fn new(session_id: SessionId) -> Self {
        Self { session_id }
    }
}

impl<TSendables> SessionMessenger<TSendables> for SelfSession {
    fn entity_added(&self, channels: &SessionChannels<TSendables>, sync_id: SyncId) {
        info!("Entity Added!");
        channels
            .entity
            .0
            .try_send(EntityActions::Add(sync_id))
            .expect("Unbounded");
    }

    fn entity_removed(&self, channels: &SessionChannels<TSendables>, sync_id: SyncId) {
        info!("Entity Removed!");
        channels
            .entity
            .0
            .try_send(EntityActions::Remove(sync_id))
            .expect("Unbounded");
    }

    fn component_added(
        &self,
        channels: &SessionChannels<TSendables>,
        external_component: ExternalComponent,
        component_data: TSendables,
    ) {
        info!("Added client component!");
        channels
            .added
            .0
            .try_send(AddedData {
                component_data,
                external_component,
            })
            .expect("Unbounded");
    }

    fn componend_updated(
        &self,
        channels: &SessionChannels<TSendables>,
        external_component: ExternalComponent,
        component_data: TSendables,
    ) {
        info!("Updated client component!");
        channels
            .updated
            .0
            .try_send(UpdatedData {
                component_data,
                external_component,
            })
            .expect("Unbounded");
    }

    fn component_removed(
        &self,
        channels: &SessionChannels<TSendables>,
        external_component: ExternalComponent,
    ) {
        info!("Removed client component!");
        channels
            .removed
            .0
            .try_send(RemovedData { external_component })
            .expect("Unbounded");
    }
}
