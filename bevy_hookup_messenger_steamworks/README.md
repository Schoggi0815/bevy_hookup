# Steamworks Messenger

[![crates.io](https://img.shields.io/crates/v/bevy_hookup_messenger_steamworks)](https://crates.io/crates/bevy_hookup_messenger_steamworks)
[![docs.rs](https://docs.rs/bevy_hookup_messenger_steamworks/badge.svg)](https://docs.rs/bevy_hookup_messenger_steamworks)

A session implementation that works over the steamworks peer 2 peer api. This allows both hosting and joining the p2p connection, but you need to collect the SteamID yourself.
If you want to be able to join other players you also have to implement the lobby yourself, this is purely for the p2p communication.
You will also need to set up bevy-steamworks yourself.

## Example

### Server

All you need to do is register the `SteamworksServerPlugin` and the `SteamworksSessionHandlerPlugin` plugin and add an entity with the `SteamworksServer` component.
You can add the component anytime, this will crate the listener for clients to connect to.
Every connected client will get its own `Connection` registered.

```rust
fn main() {
    App::new()
        .add_plugins((
            // Other plugins...
            SteamworksServerPlugin,
            SteamworksSessionHandlerPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(
        SteamworksServer::new(&client)
            .expect("Couldn't create steamworks server"),
    );
}
```

### Client

This setup is very simple, all you need to do is add the `SteamworksSessionHandlerPlugin` plugin and call the `SteamworksClient::create(&Client, SteamId, &mut Commands)` function.
This creates an entity with both the `SteamworksSessionHandler` and `Connection` component.

```rust
fn main() {
    App::new()
        .add_plugins((
            // Other plugins...
            SteamworksSessionHandlerPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, client: Res<Client>) {
    SteamworksClient::create(
        &client,
        steam_id_to_connect_to,
        &mut commands,
    ).expect("FAIL");
}
```