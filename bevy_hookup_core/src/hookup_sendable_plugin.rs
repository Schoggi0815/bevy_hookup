use bevy::prelude::*;

use crate::{
    connection::{Connection, connection_id::ConnectionId},
    entity_sharing::{entity_origin::EntityOrigin, hookup_entity_plugin::HookupEntityPlugin},
};

pub struct HookupSendablePlugin;

#[derive(SystemSet, Debug, Hash, Clone, PartialEq, Eq)]
pub struct ReadIncomingSystems;

impl Plugin for HookupSendablePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(HookupEntityPlugin)
            .add_systems(
                FixedPostUpdate,
                Self::read_incoming_messages.in_set(ReadIncomingSystems),
            )
            .add_observer(Self::remove_session);
    }
}

impl HookupSendablePlugin {
    pub fn read_incoming_messages(connections: Query<&mut Connection>) {
        for mut connection in connections {
            connection.collect_messages();
        }
    }

    pub fn remove_session(
        trigger: On<Remove, Connection>,
        connections: Query<&Connection>,
        from_sesions: Query<(Entity, &EntityOrigin<ConnectionId>)>,
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
