use std::fmt::Debug;

use bevy::prelude::*;

use crate::steamworks_server::SteamworksServer;

#[derive(Debug)]
pub struct SteamworksServerPlugin;

impl Plugin for SteamworksServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, SteamworksServer::handle_events);
    }
}
