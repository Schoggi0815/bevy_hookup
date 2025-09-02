use bevy::prelude::*;
use bevy_hookup_core::{
    external_entity::ExternalEntity,
    hook_session::{SessionId, SessionMessager},
    session::{AddedData, RemovedData, SessionChannels, UpdatedData},
};

pub struct SelfSession {
    session_id: SessionId,
}

impl SelfSession {
    pub fn new(session_id: SessionId) -> Self {
        Self { session_id }
    }
}

impl<TSendables> SessionMessager<TSendables> for SelfSession {
    fn component_added(
        &self,
        channels: &SessionChannels<TSendables>,
        entity: Entity,
        component_data: TSendables,
    ) {
        info!("Added client component!");
        channels
            .added
            .0
            .try_send(AddedData {
                component_data,
                entity: ExternalEntity::new(entity, self.session_id),
            })
            .expect("Unbounded");
    }

    fn componend_updated(
        &self,
        channels: &SessionChannels<TSendables>,
        entity: Entity,
        component_data: TSendables,
    ) {
        info!("Updated client component!");
        channels
            .updated
            .0
            .try_send(UpdatedData {
                component_data,
                entity: ExternalEntity::new(entity, self.session_id),
            })
            .expect("Unbounded");
    }

    fn component_removed(&self, channels: &SessionChannels<TSendables>, entity: Entity) {
        info!("Removed client component!");
        channels
            .removed
            .0
            .try_send(RemovedData {
                entity: ExternalEntity::new(entity, self.session_id),
            })
            .expect("Unbounded");
    }
}
