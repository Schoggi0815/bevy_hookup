use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{
    from_session::FromSession, hookup_entity_plugin::HookupEntityPlugin,
    send_entity_systems::SendEntitySystems, session::Session,
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
    pub fn send_session_messages(sessions: Query<&mut Session<TSendables>>) {
        for mut session in sessions {
            session.push_messages();
        }
    }

    pub fn remove_session(
        trigger: On<Remove, Session<TSendables>>,
        sessions: Query<&Session<TSendables>>,
        from_sesions: Query<(Entity, &FromSession)>,
        mut commands: Commands,
    ) {
        let Ok(removed_session) = sessions.get(trigger.entity) else {
            warn!("Removed session not found!");
            return;
        };

        let session_id = removed_session.get_session_id();

        for (from_entity, _) in from_sesions
            .iter()
            .filter(|fs| fs.1.session_id == session_id)
        {
            commands.entity(from_entity).despawn();
        }
    }
}
