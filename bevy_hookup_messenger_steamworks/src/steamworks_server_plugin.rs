use std::{fmt::Debug, marker::PhantomData};

use bevy::prelude::*;
use serde::{Serialize, de::DeserializeOwned};

use crate::steamworks_server::SteamworksServer;

#[derive(Debug)]
pub struct SteamworksServerPlugin<TSendables>(PhantomData<TSendables>);

impl<TSendables> Default for SteamworksServerPlugin<TSendables> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone> Plugin
    for SteamworksServerPlugin<TSendables>
{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, SteamworksServer::<TSendables>::handle_events);
    }
}
