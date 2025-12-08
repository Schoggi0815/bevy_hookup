use bevy::prelude::*;
use bevy_steamworks::SteamId;

#[derive(Component, Debug)]
pub struct SteamReference(pub SteamId);
