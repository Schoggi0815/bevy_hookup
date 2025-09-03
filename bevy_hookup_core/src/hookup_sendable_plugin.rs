use std::marker::PhantomData;

use bevy::app::Plugin;

use crate::hookup_entity_plugin::HookupEntityPlugin;

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
        app.add_plugins(HookupEntityPlugin::<TSendables>::default());
    }
}
