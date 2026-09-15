use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{
    connection::{Connection, remote_action::RemoteAction},
    entity_sharing::{
        receive_entity_systems::ReceiveEntitySystems,
        send_entity_systems::SendEntitySystems,
        sync_entity::{SyncEntity, SyncEntityOwner},
    },
    origin::Origin,
};

pub struct HookupEntityPlugin<TSendables: Send + Sync + 'static + Clone> {
    _phantom_sendable: PhantomData<TSendables>,
}

impl<TSendables: Send + Sync + 'static + Clone> Default for HookupEntityPlugin<TSendables> {
    fn default() -> Self {
        Self {
            _phantom_sendable: Default::default(),
        }
    }
}

impl<TSendables: Send + Sync + 'static + Clone> Plugin for HookupEntityPlugin<TSendables> {
    fn build(&self, app: &mut bevy::app::App) {
        app.register_type::<SyncEntity>()
            .register_type::<SyncEntityOwner>()
            .add_systems(
                FixedUpdate,
                (send_entites::<TSendables>, init_session::<TSendables>)
                    .in_set(SendEntitySystems::<TSendables>::default()),
            )
            .add_systems(
                FixedUpdate,
                (check_entity_channel::<TSendables>,)
                    .in_set(ReceiveEntitySystems::<TSendables>::default()),
            )
            .add_observer(send_removed_entites::<TSendables>);
    }
}

fn send_removed_entites<TSendables: Send + Sync + 'static + Clone>(
    trigger: On<Remove, SyncEntityOwner>,
    sync_entities: Query<(&SyncEntity, &SyncEntityOwner)>,
    connections: Query<&mut Connection<TSendables>>,
) {
    let Ok((removed_entity, removed_owner)) = sync_entities.get(trigger.entity) else {
        warn!("Couldn't find removed sync entity.");
        return;
    };

    for mut connection in connections {
        if !removed_owner
            .session_read_filter
            .is_allowed(&connection.get_connection_id())
        {
            continue;
        }

        connection.entity_removed(removed_entity.sync_id);
    }
}

fn send_entites<TSendables: Send + Sync + 'static + Clone>(
    mut connections: Query<&mut Connection<TSendables>>,
    sync_entities: Query<(&mut SyncEntityOwner, &SyncEntity), Changed<SyncEntityOwner>>,
) {
    for (mut owner, sync) in sync_entities {
        for mut session in connections.iter_mut() {
            let session_id = session.get_connection_id();
            let in_session = owner.on_sessions.contains(&session_id);
            let allowed_in_session = owner.session_read_filter.is_allowed(&session_id);
            if in_session && !allowed_in_session {
                session.entity_removed(sync.sync_id);
                owner.on_sessions = owner
                    .on_sessions
                    .clone()
                    .into_iter()
                    .filter(|sid| *sid != session_id)
                    .collect();
            } else if !in_session && allowed_in_session {
                session.entity_added(sync.sync_id);
                owner.on_sessions.push(session_id);
            }
        }
    }
}

fn init_session<TSendables: Send + Sync + 'static + Clone>(
    connections: Query<&mut Connection<TSendables>, Added<Connection<TSendables>>>,
    mut sync_entities: Query<(&mut SyncEntityOwner, &SyncEntity)>,
) {
    for mut connection in connections {
        for (mut owner, sync) in sync_entities.iter_mut() {
            if !owner
                .session_read_filter
                .is_allowed(&connection.get_connection_id())
            {
                continue;
            }

            connection.entity_added(sync.sync_id);
            owner.on_sessions.push(connection.get_connection_id());
        }
    }
}

fn check_entity_channel<TSendables: Send + Sync + 'static + Clone>(
    connections: Query<&Connection<TSendables>>,
    mut commands: Commands,
    sync_entities: Query<(Entity, &SyncEntity)>,
) {
    for connection in connections {
        let mut unused_actions = Vec::new();
        for session_action in connection.channels.receiver.try_iter() {
            match session_action {
                RemoteAction::AddEntity { id } => {
                    if sync_entities.iter().find(|se| se.1.sync_id == id).is_some() {
                        continue;
                    }

                    commands.spawn((
                        SyncEntity::new_from_id(id),
                        Origin(connection.get_connection_id()),
                    ));
                }
                RemoteAction::RemoveEntity { id } => {
                    let Some((sync_entity, _)) = sync_entities.iter().find(|se| se.1.sync_id == id)
                    else {
                        continue;
                    };

                    commands.entity(sync_entity).despawn();
                }
                _ => {
                    unused_actions.push(session_action);
                }
            }
        }
        unused_actions
            .into_iter()
            .for_each(|sa| connection.channels.sender.try_send(sa).expect("unbounded"));
    }
}
