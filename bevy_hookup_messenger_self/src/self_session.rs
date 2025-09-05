use bevy::prelude::*;
use bevy_hookup_core::{
    external_component::ExternalComponent,
    hook_session::{SessionId, SessionMessenger},
    session::{AddedData, EntityActions, RemovedData, Session, SessionChannels, UpdatedData},
    sync_id::SyncId,
};
use crossbeam::channel::unbounded;

pub struct SelfSession<TSendables: Clone> {
    session_id: SessionId,
    channels: SessionChannels<TSendables>,
}

impl<TSendables: Clone> SelfSession<TSendables> {
    pub fn new() -> Self {
        Self {
            session_id: SessionId::default(),
            channels: SessionChannels {
                added: unbounded(),
                entity: unbounded(),
                removed: unbounded(),
                updated: unbounded(),
            },
        }
    }
}

impl<TSendables: Clone + Send + Sync + 'static> SessionMessenger<TSendables>
    for SelfSession<TSendables>
{
    fn to_session(self) -> Session<TSendables> {
        let channels = self.channels.clone();
        Session::new(Box::new(self), channels)
    }

    fn entity_added(&mut self, channels: &SessionChannels<TSendables>, sync_id: SyncId) {
        info!("Entity Added!");
        channels
            .entity
            .0
            .try_send(EntityActions::Add(sync_id.counterpart()))
            .expect("Unbounded");
    }

    fn entity_removed(&mut self, channels: &SessionChannels<TSendables>, sync_id: SyncId) {
        info!("Entity Removed!");
        channels
            .entity
            .0
            .try_send(EntityActions::Remove(sync_id.counterpart()))
            .expect("Unbounded");
    }

    fn component_added(
        &mut self,
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
                external_component: external_component.counterpart(),
            })
            .expect("Unbounded");
    }

    fn componend_updated(
        &mut self,
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
                external_component: external_component.counterpart(),
            })
            .expect("Unbounded");
    }

    fn component_removed(
        &mut self,
        channels: &SessionChannels<TSendables>,
        external_component: ExternalComponent,
    ) {
        info!("Removed client component!");
        channels
            .removed
            .0
            .try_send(RemovedData {
                external_component: external_component.counterpart(),
            })
            .expect("Unbounded");
    }

    fn get_session_id(&self) -> SessionId {
        self.session_id
    }

    fn get_channels(&self) -> SessionChannels<TSendables> {
        self.channels.clone()
    }
}
