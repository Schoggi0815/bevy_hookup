use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{
    from_session::FromSession, hookup_entity_plugin::HookupEntityPlugin, session::Session,
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
            .add_systems(Update, Self::remove_session);
    }
}

impl<TSendables: Send + Sync + 'static + Clone> HookupSendablePlugin<TSendables> {
    pub fn remove_session(
        sessions: Query<(Entity, &Session<TSendables>), Changed<Session<TSendables>>>,
        from_sesions: Query<(Entity, &FromSession)>,
        mut commands: Commands,
    ) {
        for (session_entity, session) in sessions {
            if !session.remove {
                continue;
            }

            let session_id = session.get_session_id();

            commands.entity(session_entity).despawn();

            for (from_entity, _) in from_sesions
                .iter()
                .filter(|fs| fs.1.session_id == session_id)
            {
                commands.entity(from_entity).despawn();
            }
        }
    }
}
