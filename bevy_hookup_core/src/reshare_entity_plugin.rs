use bevy::prelude::*;

use std::marker::PhantomData;

use crate::{
    from_session::FromSession,
    receive_entity_systems::ReceiveEntitySystems,
    reshare_entity_component::ReshareEntityComponent,
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
        app.add_systems(
            Update,
            Self::reshare_entity.after(ReceiveEntitySystems::<TS>::default()),
        );
    }
}

impl<TS> ReshareEntityPlugin<TS> {
    fn reshare_entity(
        missing_owners: Query<
            (Entity, &FromSession),
            (
                With<ReshareEntityComponent>,
                With<SyncEntity>,
                Without<SyncEntityOwner>,
            ),
        >,
        mut commands: Commands,
    ) where
        TS: Send + Sync + 'static,
    {
        for (entity, from_session) in missing_owners {
            commands.entity(entity).insert(
                SyncEntityOwner::new()
                    .with_read_filter(SessionFilter::BlacklistReshare(vec![
                        from_session.session_id,
                    ]))
                    .with_write_filter(SessionFilter::Whitelist(vec![from_session.session_id])),
            );
        }
    }
}
