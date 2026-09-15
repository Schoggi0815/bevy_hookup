use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{
    connection::{Connection, connection_id::ConnectionId},
    entity_sharing::hookup_entity_plugin::HookupEntityPlugin,
    origin::Origin,
};

pub struct HookupSendablePlugin<TSendables: Send + Sync + 'static + Clone> {
    _phantom: PhantomData<TSendables>,
}

#[derive(SystemSet, Debug, Hash, Clone, PartialEq, Eq)]
pub struct ReadIncomingSystems;

impl<TSendables: Send + Sync + 'static + Clone> Default for HookupSendablePlugin<TSendables> {
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}

impl<TSendables: Send + Sync + 'static + Clone> Plugin for HookupSendablePlugin<TSendables> {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(HookupEntityPlugin::<TSendables>::default())
            .add_systems(
                FixedPostUpdate,
                Self::read_incoming_messages.in_set(ReadIncomingSystems),
            )
            .add_observer(Self::remove_session);
    }
}

impl<TSendables: Send + Sync + 'static + Clone> HookupSendablePlugin<TSendables> {
    pub fn read_incoming_messages(connections: Query<&mut Connection<TSendables>>) {
        for mut connection in connections {
            connection.collect_messages();
        }
    }

    pub fn remove_session(
        trigger: On<Remove, Connection<TSendables>>,
        connections: Query<&Connection<TSendables>>,
        from_sesions: Query<(Entity, &Origin<ConnectionId>)>,
        mut commands: Commands,
    ) {
        let Ok(removed_connection) = connections.get(trigger.entity) else {
            warn!("Removed session not found!");
            return;
        };

        let session_id = removed_connection.get_connection_id();

        for (from_entity, _) in from_sesions.iter().filter(|(_, o)| o.0 == session_id) {
            commands.entity(from_entity).despawn();
        }
    }
}
