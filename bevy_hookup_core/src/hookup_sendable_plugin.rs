use std::marker::PhantomData;

use bevy::app::Plugin;

use crate::session_handler::SessionHandler;

#[derive(Default)]
pub struct HookupSendablePlugin<TSendables: Send + Sync + 'static> {
    _phantom: PhantomData<TSendables>,
}

impl<TSendables: Send + Sync + 'static> Plugin for HookupSendablePlugin<TSendables> {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(SessionHandler::<TSendables>::new());
    }
}
