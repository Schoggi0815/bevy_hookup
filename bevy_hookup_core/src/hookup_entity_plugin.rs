use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{
    from_session::FromSession,
    session::Session,
    session_action::SessionAction,
    sync_entity::{SyncEntity, SyncEntityOwner},
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
                FixedPreUpdate,
                (
                    send_entites::<TSendables>,
                    init_session::<TSendables>,
                    check_entity_channel::<TSendables>,
                ),
            )
            .add_observer(send_removed_entites::<TSendables>);
    }
}

fn send_removed_entites<TSendables: Send + Sync + 'static + Clone>(
    trigger: Trigger<OnRemove, SyncEntityOwner>,
    sync_entities: Query<(&SyncEntity, &SyncEntityOwner)>,
    sessions: Query<&mut Session<TSendables>>,
) {
    let Ok((removed_entity, removed_owner)) = sync_entities.get(trigger.target()) else {
        warn!("Couldn't find removed sync entity.");
        return;
    };

    for mut session in sessions {
        if !removed_owner
            .session_read_filter
            .allow_session(&session.get_session_id())
        {
            continue;
        }

        session.entity_removed(removed_entity.sync_id);
    }
}

fn send_entites<TSendables: Send + Sync + 'static + Clone>(
    mut sessions: Query<&mut Session<TSendables>>,
    sync_entities: Query<(&mut SyncEntityOwner, &SyncEntity), Changed<SyncEntityOwner>>,
) {
    for (mut owner, sync) in sync_entities {
        for mut session in sessions.iter_mut() {
            let session_id = session.get_session_id();
            let in_session = owner.on_sessions.contains(&session_id);
            let allowed_in_session = owner.session_read_filter.allow_session(&session_id);
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
    sessions: Query<&mut Session<TSendables>, Added<Session<TSendables>>>,
    mut sync_entities: Query<(&mut SyncEntityOwner, &SyncEntity)>,
) {
    for mut session in sessions {
        for (mut owner, sync) in sync_entities.iter_mut() {
            if !owner
                .session_read_filter
                .allow_session(&session.get_session_id())
            {
                continue;
            }

            session.entity_added(sync.sync_id);
            owner.on_sessions.push(session.get_session_id());
        }
    }
}

fn check_entity_channel<TSendables: Send + Sync + 'static + Clone>(
    sessions: Query<&Session<TSendables>>,
    mut commands: Commands,
    sync_entities: Query<(Entity, &SyncEntity)>,
) {
    for session in sessions {
        let mut unused_actions = Vec::new();
        for session_action in session.channels.receiver.try_iter() {
            match session_action {
                SessionAction::AddEntity { id } => {
                    if sync_entities.iter().find(|se| se.1.sync_id == id).is_some() {
                        continue;
                    }

                    commands.spawn((
                        SyncEntity::new_from_id(id),
                        FromSession {
                            session_id: session.get_session_id(),
                        },
                    ));
                }
                SessionAction::RemoveEntity { id } => {
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
            .for_each(|sa| session.channels.sender.try_send(sa).expect("unbounded"));
    }
}
