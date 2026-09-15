use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{
    connection::{Connection, connection_id::ConnectionId},
    entity_sharing::{
        hookup_entity_plugin::HookupEntityPlugin, send_entity_systems::SendEntitySystems,
    },
    origin::Origin,
};

pub struct HookupSendablePlugin<TSendables: Send + Sync + 'static + Clone> {
    _phantom: PhantomData<TSendables>,
}

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
                Self::send_session_messages.in_set(SendEntitySystems::<TSendables>::default()),
            )
            .add_observer(Self::remove_session);
    }
}

impl<TSendables: Send + Sync + 'static + Clone> HookupSendablePlugin<TSendables> {
    pub fn send_session_messages(connections: Query<&mut Connection<TSendables>>) {
        for mut connections in connections {
            connections.push_messages();
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
