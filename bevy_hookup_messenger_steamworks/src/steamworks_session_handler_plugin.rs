use bevy::prelude::*;
use bevy_hookup_core::{
    receive_entity_systems::ReceiveEntitySystems, send_entity_systems::SendEntitySystems,
};
use serde::{Serialize, de::DeserializeOwned};
use std::marker::PhantomData;

use crate::steamworks_session_handler::SteamworksSessionHandler;

pub struct SteamworksSessionHandlerPlugin<TSendables>(PhantomData<TSendables>);

impl<T> Default for SteamworksSessionHandlerPlugin<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone> Plugin
    for SteamworksSessionHandlerPlugin<TSendables>
{
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            FixedPostUpdate,
            Self::send_handler_actions.after(SendEntitySystems::<TSendables>::default()),
        )
        .add_systems(
            FixedUpdate,
            Self::check_handler_actions.before(ReceiveEntitySystems::<TSendables>::default()),
        );
    }
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone>
    SteamworksSessionHandlerPlugin<TSendables>
{
    fn send_handler_actions(handlers: Query<&SteamworksSessionHandler<TSendables>>) {
        for handler in handlers {
            handler.send_actions();
        }
    }

    fn check_handler_actions(handlers: Query<&mut SteamworksSessionHandler<TSendables>>) {
        for mut handler in handlers {
            handler.check_actions();
        }
    }
}
