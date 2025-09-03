use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{
    session::{EntityActions, Session},
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
                    check_entity_channel::<TSendables>,
                ),
            );
    }
}

fn send_entites<TSendables: Send + Sync + 'static + Clone>(
    mut sessions: Query<&mut Session<TSendables>>,
    sync_entities_added: Query<&SyncEntity, Added<SyncEntityOwner>>,
    sync_entities_changed: Query<(Entity, &SyncEntityOwner, &SyncEntity), Changed<SyncEntityOwner>>,
    mut commands: Commands,
) {
    for added_entity in sync_entities_added {
        for mut session in sessions.iter_mut() {
            session.entity_added(added_entity.sync_id);
        }
    }

    for (changed_entity, changed_owner, changed_sync) in sync_entities_changed {
        if !changed_owner.remove {
            continue;
        }

        for mut session in sessions.iter_mut() {
            session.entity_removed(changed_sync.sync_id);
        }

        commands.entity(changed_entity).despawn();
    }
}

fn check_entity_channel<TSendables: Send + Sync + 'static + Clone>(
    sessions: Query<&Session<TSendables>>,
    mut commands: Commands,
    sync_entities: Query<(Entity, &SyncEntity)>,
) {
    for session in sessions {
        for entity_action in session.channels.entity.1.try_iter() {
            match entity_action {
                EntityActions::Add(sync_id) => {
                    if sync_entities
                        .iter()
                        .find(|se| se.1.sync_id == sync_id)
                        .is_some()
                    {
                        continue;
                    }

                    commands.spawn(SyncEntity::new_from_id(sync_id));
                }
                EntityActions::Remove(sync_id) => {
                    let Some((sync_entity, _)) =
                        sync_entities.iter().find(|se| se.1.sync_id == sync_id)
                    else {
                        continue;
                    };

                    commands.entity(sync_entity).despawn();
                }
            }
        }
    }
}
