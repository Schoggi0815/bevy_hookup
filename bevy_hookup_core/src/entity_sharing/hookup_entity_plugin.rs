use bevy::{ecs::entity_disabling::Disabled, prelude::*};

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
        sync_entity::{EntityReadFilter, EntityWriteFilter, SyncEntity, SyncEntityOwner},
    },
    hookup_core_plugin::ReadIncomingSystems,
};

pub struct HookupEntityPlugin;

impl Plugin for HookupEntityPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.register_type::<EntityOrigin<ConnectionId>>()
            .register_type::<EntityOrigin<ClientId>>()
            .register_type::<EntityReadFilter<ClientId>>()
            .register_type::<EntityReadFilter<ConnectionId>>()
            .register_type::<EntityWriteFilter<ClientId>>()
            .register_type::<EntityWriteFilter<ConnectionId>>()
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
    sync_entities: Query<
        (
            &SyncEntity,
            Option<&EntityOrigin<ClientId>>,
            &EntityReadFilter<ConnectionId>,
        ),
        (With<SyncEntityOwner>, Allow<Disabled>),
    >,
    connections: Query<&mut Connection>,
    client_id: Res<ClientId>,
) -> Result {
    let Ok((removed_entity, entity_origin, connection_filter)) = sync_entities.get(trigger.entity)
    else {
        warn!("Couldn't find removed sync entity.");
        return Ok(());
    };

    for mut connection in connections {
        if !connection_filter.is_allowed(&connection.get_connection_id()) {
            continue;
        }

        let origin = entity_origin.map_or(*client_id, |eo| eo.0);

        connection.entity_removed(removed_entity.sync_id, origin)?;
    }

    Ok(())
}

fn send_entites(
    mut connections: Query<&mut Connection>,
    sync_entities: Query<
        (
            &mut SyncEntityOwner,
            &SyncEntity,
            Option<&EntityOrigin<ClientId>>,
            &EntityReadFilter<ConnectionId>,
            &EntityReadFilter<ClientId>,
            &EntityWriteFilter<ClientId>,
        ),
        (
            Or<(
                Changed<EntityReadFilter<ConnectionId>>,
                Changed<EntityReadFilter<ClientId>>,
                Changed<EntityWriteFilter<ClientId>>,
            )>,
            Allow<Disabled>,
        ),
    >,
    client_id: Res<ClientId>,
) -> Result {
    for (
        mut owner,
        sync,
        origin_client,
        connection_filter,
        client_read_filter,
        client_write_filter,
    ) in sync_entities
    {
        let origin = origin_client.map_or(*client_id, |oc| oc.0);

        for mut session in connections.iter_mut() {
            let session_id = session.get_connection_id();
            let in_session = owner.on_connections.contains(&session_id);
            let allowed_in_session = connection_filter.is_allowed(&session_id);
            if in_session && !allowed_in_session {
                session.entity_removed(sync.sync_id, origin)?;
                owner.on_connections = owner
                    .on_connections
                    .clone()
                    .into_iter()
                    .filter(|sid| *sid != session_id)
                    .collect();
            } else if !in_session && allowed_in_session {
                session.entity_added(
                    sync.sync_id,
                    origin,
                    client_read_filter.0.clone(),
                    client_write_filter.0.clone(),
                )?;
                owner.on_connections.push(session_id);
            } else if in_session && allowed_in_session {
                session.entity_updated(
                    sync.sync_id,
                    origin,
                    client_read_filter.0.clone(),
                    client_write_filter.0.clone(),
                )?;
            }
        }
    }

    Ok(())
}

fn init_session(
    connections: Query<&mut Connection, Added<Connection>>,
    mut sync_entities: Query<
        (
            &mut SyncEntityOwner,
            &SyncEntity,
            Option<&EntityOrigin<ClientId>>,
            &EntityReadFilter<ConnectionId>,
            &EntityReadFilter<ClientId>,
            &EntityWriteFilter<ClientId>,
        ),
        Allow<Disabled>,
    >,
    client_id: Res<ClientId>,
) -> Result {
    for mut connection in connections {
        for (
            mut owner,
            sync,
            origin_client,
            connection_filter,
            client_read_filter,
            client_write_filter,
        ) in sync_entities.iter_mut()
        {
            if !connection_filter.is_allowed(&connection.get_connection_id()) {
                continue;
            }

            let origin = origin_client.map_or(*client_id, |oc| oc.0);

            connection.entity_added(
                sync.sync_id,
                origin,
                client_read_filter.0.clone(),
                client_write_filter.0.clone(),
            )?;
            owner.on_connections.push(connection.get_connection_id());
        }
    }

    Ok(())
}

fn check_entity_channel(
    connections: Query<&Connection>,
    mut commands: Commands,
    sync_entities: Query<
        (
            Entity,
            &SyncEntity,
            &EntityReadFilter<ClientId>,
            &EntityWriteFilter<ClientId>,
        ),
        Allow<Disabled>,
    >,
    client_id: Res<ClientId>,
) {
    for connection in connections {
        for (action, id, origin_client_id) in connection.messages().filter_map(|ra| match ra {
            RemoteAction::Entity {
                action,
                id,
                client_id,
            } => Some((action, id, client_id)),
            _ => None,
        }) {
            let sync_entity = sync_entities
                .iter()
                .find(|se| &se.1.sync_id == id)
                .map(|(e, _, rf, wf)| (e, rf, wf));

            match action {
                EntityAction::AddOrUpdate {
                    client_read_filter,
                    client_write_filter,
                } => {
                    if let Some((entity, read_filter, write_filter)) = sync_entity {
                        if (client_read_filter != &read_filter.0)
                            || (client_write_filter != &write_filter.0)
                        {
                            commands.entity(entity).insert((
                                EntityReadFilter(client_read_filter.clone()),
                                EntityWriteFilter(client_write_filter.clone()),
                            ));

                            if client_read_filter.is_allowed(&client_id) {
                                commands.entity(entity).remove::<Disabled>();
                            } else {
                                commands.entity(entity).insert(Disabled);
                            }
                        }

                        continue;
                    }

                    let mut entity_commands = commands.spawn((
                        SyncEntity::new_from_id(*id),
                        EntityOrigin(connection.get_connection_id()),
                        EntityOrigin(*origin_client_id),
                        EntityReadFilter(client_read_filter.clone()),
                        EntityWriteFilter(client_write_filter.clone()),
                    ));

                    if !client_read_filter.is_allowed(&client_id) {
                        entity_commands.insert(Disabled);
                    }
                }
                EntityAction::Remove => {
                    let Some((sync_entity, ..)) = sync_entity else {
                        warn!("Entity to remove doesn't exist");
                        continue;
                    };

                    commands.entity(sync_entity).despawn();
                }
            }
        }
    }
}
