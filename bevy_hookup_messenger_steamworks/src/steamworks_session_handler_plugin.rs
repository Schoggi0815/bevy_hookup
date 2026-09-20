use bevy::prelude::*;
use bevy_hookup_core::entity_sharing::{
    receive_entity_systems::ReceiveEntitySystems, send_entity_systems::SendEntitySystems,
};

use crate::steamworks_session_handler::SteamworksSessionHandler;

pub struct SteamworksSessionHandlerPlugin;

impl Plugin for SteamworksSessionHandlerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            FixedPostUpdate,
            Self::send_handler_actions.after(SendEntitySystems),
        )
        .add_systems(
            FixedUpdate,
            Self::check_handler_actions.before(ReceiveEntitySystems),
        );
    }
}

impl SteamworksSessionHandlerPlugin {
    fn send_handler_actions(handlers: Query<&SteamworksSessionHandler>) {
        for handler in handlers {
            handler.send_actions();
        }
    }

    fn check_handler_actions(handlers: Query<&mut SteamworksSessionHandler>) {
        for mut handler in handlers {
            handler.check_actions();
        }
    }
}
