use bevy::prelude::*;

use std::marker::PhantomData;

use crate::{
    from_session::FromSession,
    hookup_entity_systems::HookupEntitySystems,
    session::Session,
    session_filter::SessionFilter,
    sync_entity::{SyncEntity, SyncEntityOwner},
};

pub struct ReshareEntityPlugin<TSendables>(PhantomData<TSendables>);

impl<TS> Default for ReshareEntityPlugin<TS> {
    fn default() -> Self {
        Self(PhantomData::default())
    }
}

impl<TS> Plugin for ReshareEntityPlugin<TS>
where
    TS: Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::reshare_entity).add_systems(
            FixedPreUpdate,
            Self::update_new_session.before(HookupEntitySystems::<TS>::default()),
        );
    }
}

impl<TS> ReshareEntityPlugin<TS> {
    fn reshare_entity(
        added_entities: Query<(Entity, &FromSession), Added<SyncEntity>>,
        sessions: Query<&Session<TS>>,
        mut commands: Commands,
    ) where
        TS: Send + Sync + 'static,
    {
        let same_world_sessions = sessions
            .iter()
            .filter(|s| s.pushes_to_same_world())
            .map(|s| s.get_session_id())
            .collect::<Vec<_>>();

        for (entity, from_session) in added_entities {
            if same_world_sessions.contains(&from_session.session_id) {
                continue;
            }

            commands.entity(entity).insert(
                SyncEntityOwner::new()
                    .with_read_filter(SessionFilter::BlacklistReshare(
                        same_world_sessions
                            .clone()
                            .into_iter()
                            .chain([from_session.session_id])
                            .collect(),
                    ))
                    .with_write_filter(SessionFilter::Whitelist(vec![from_session.session_id])),
            );
        }
    }

    fn update_new_session(
        added_sessions: Query<&Session<TS>, Added<Session<TS>>>,
        mut entity_owners: Query<&mut SyncEntityOwner>,
    ) where
        TS: Send + Sync + 'static,
    {
        for added_session in added_sessions {
            if !added_session.pushes_to_same_world() {
                continue;
            }

            for mut entity_owner in entity_owners.iter_mut() {
                match &mut entity_owner.session_read_filter {
                    SessionFilter::BlacklistReshare(blacklist) => {
                        blacklist.push(added_session.get_session_id());
                    }
                    _ => continue,
                }
            }
        }
    }
}
