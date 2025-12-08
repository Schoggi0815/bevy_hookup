use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_hookup_core::hook_session::SessionMessenger;
use bevy_steamworks::{
    Client, SteamId, networking_sockets::InvalidHandle, networking_types::NetworkingIdentity,
};
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    steam_reference::SteamReference, steamworks_session_handler::SteamworksSessionHandler,
};

#[derive(Component)]
pub struct SteamworksClient<TSendables> {
    phamtom: PhantomData<TSendables>,
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone + Sized>
    SteamworksClient<TSendables>
{
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

        let (handler, session) = SteamworksSessionHandler::<TSendables>::new_pair(connection);

        commands.spawn((SteamReference(steam_user), session.to_session(), handler));

        Ok(())
    }
}
