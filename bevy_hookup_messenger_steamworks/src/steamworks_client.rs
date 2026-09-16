use bevy::prelude::*;
use bevy_hookup_core::connection::connection_messenger::ConnectionMessenger;
use bevy_steamworks::{
    Client, SteamId, networking_sockets::InvalidHandle, networking_types::NetworkingIdentity,
};

use crate::{
    steam_reference::SteamReference, steamworks_session_handler::SteamworksSessionHandler,
};

#[derive(Component)]
pub struct SteamworksClient;

impl SteamworksClient {
    pub fn create(
        client: &Client,
        steam_user: SteamId,
        commands: &mut Commands,
    ) -> Result<(), InvalidHandle> {
        let connection = client.networking_sockets().connect_p2p(
            NetworkingIdentity::new_steam_id(steam_user),
            0,
            [],
        )?;

        let (handler, session) = SteamworksSessionHandler::new_pair(connection);

        commands.spawn((SteamReference(steam_user), session.to_connection(), handler));

        Ok(())
    }
}
