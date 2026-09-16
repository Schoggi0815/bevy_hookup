use bevy::prelude::*;

use crate::{
    client_id::ClientId,
    connection::{
        Connection,
        connection_id::ConnectionId,
        remote_action::{EntityAction, RemoteAction},
    },
    entity_sharing::{
        entity_origin::EntityOrigin,
        receive_entity_systems::ReceiveEntitySystems,
        send_entity_systems::SendEntitySystems,
        sync_entity::{SyncEntity, SyncEntityOwner},
    },
    hookup_sendable_plugin::ReadIncomingSystems,
};

pub struct HookupEntityPlugin;

impl Plugin for HookupEntityPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.register_type::<EntityOrigin<ConnectionId>>()
            .register_type::<EntityOrigin<ClientId>>()
            .add_systems(
                FixedUpdate,
                (send_entites, init_session).in_set(SendEntitySystems),
            )
            .add_systems(
                FixedUpdate,
                check_entity_channel.in_set(ReceiveEntitySystems),
            )
            .add_observer(send_removed_entites)
            .configure_sets(FixedUpdate, ReceiveEntitySystems.after(ReadIncomingSystems));
    }
}

fn send_removed_entites(
    trigger: On<Remove, SyncEntityOwner>,
    sync_entities: Query<(&SyncEntity, &SyncEntityOwner)>,
    connections: Query<&mut Connection>,
) -> Result {
    let Ok((removed_entity, removed_owner)) = sync_entities.get(trigger.entity) else {
        warn!("Couldn't find removed sync entity.");
        return Ok(());
    };

    for mut connection in connections {
        if !removed_owner
            .session_read_filter
            .is_allowed(&connection.get_connection_id())
        {
            continue;
        }

        connection.entity_removed(removed_entity.sync_id)?;
    }

    Ok(())
}

fn send_entites(
    mut connections: Query<&mut Connection>,
    sync_entities: Query<(&mut SyncEntityOwner, &SyncEntity), Changed<SyncEntityOwner>>,
) -> Result {
    for (mut owner, sync) in sync_entities {
        for mut session in connections.iter_mut() {
            let session_id = session.get_connection_id();
            let in_session = owner.on_sessions.contains(&session_id);
            let allowed_in_session = owner.session_read_filter.is_allowed(&session_id);
            if in_session && !allowed_in_session {
                session.entity_removed(sync.sync_id)?;
                owner.on_sessions = owner
                    .on_sessions
                    .clone()
                    .into_iter()
                    .filter(|sid| *sid != session_id)
                    .collect();
            } else if !in_session && allowed_in_session {
                session.entity_added(sync.sync_id)?;
                owner.on_sessions.push(session_id);
            }
        }
    }

    Ok(())
}

fn init_session(
    connections: Query<&mut Connection, Added<Connection>>,
    mut sync_entities: Query<(&mut SyncEntityOwner, &SyncEntity)>,
) -> Result {
    for mut connection in connections {
        for (mut owner, sync) in sync_entities.iter_mut() {
            if !owner
                .session_read_filter
                .is_allowed(&connection.get_connection_id())
            {
                continue;
            }

            connection.entity_added(sync.sync_id)?;
            owner.on_sessions.push(connection.get_connection_id());
        }
    }

    Ok(())
}

fn check_entity_channel(
    connections: Query<&Connection>,
    mut commands: Commands,
    sync_entities: Query<(Entity, &SyncEntity)>,
) {
    for connection in connections {
        for (action, id) in connection.messages().filter_map(|ra| match ra {
            RemoteAction::Entity { action, id } => Some((action, id)),
            _ => None,
        }) {
            let sync_entity = sync_entities
                .iter()
                .find(|se| &se.1.sync_id == id)
                .map(|(e, _)| e);

            match action {
                EntityAction::Add => {
                    if sync_entity.is_some() {
                        warn!("Entity to add already exists");
                        continue;
                    }

                    commands.spawn((
                        SyncEntity::new_from_id(*id),
                        EntityOrigin(connection.get_connection_id()),
                    ));
                }
                EntityAction::Remove => {
                    let Some(sync_entity) = sync_entity else {
                        warn!("Entity to remove doesn't exist");
                        continue;
                    };

                    commands.entity(sync_entity).despawn();
                }
            }
        }
    }
}
