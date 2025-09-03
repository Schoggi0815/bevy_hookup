use std::marker::PhantomData;

use bevy::app::Plugin;

use crate::{hookup_entity_plugin::HookupEntityPlugin, session_handler::SessionHandler};

pub struct HookupSendablePlugin<TSendables: Send + Sync + 'static> {
    _phantom: PhantomData<TSendables>,
}

impl<TSendables: Send + Sync + 'static> Default for HookupSendablePlugin<TSendables> {
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}

impl<TSendables: Send + Sync + 'static> Plugin for HookupSendablePlugin<TSendables> {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(HookupEntityPlugin::<TSendables>::default())
            .insert_resource(SessionHandler::<TSendables>::new());
    }
}
