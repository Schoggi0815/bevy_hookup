# Steamworks Messenger

A session implementation that works over the steamworks peer 2 peer api. This allows both hosting and joining the p2p sessoin, but you need to collect the SteamID yourself.
If you want to be able to join other players you also have to implement the lobby yourself, this is purely for the p2p communication.
You will also need to set up bevy-steamworks yourself.

## Example

### Server

All you need to do is register the `SteamworksServerPlugin::<Sendables>` and the `SteamworksSessionHandlerPlugin::<Sendables>` plugin and add an entity with the `SteamworksServer::<Sendables>` component.
You can add the component anytime, this will crate the listener for clients to connect to.
Every connected client will get its own `Session` registered.

```rust
fn main() {
    App::new()
        .add_plugins((
            // Other plugins...
            SteamworksServerPlugin::<Sendables>::default(),
            SteamworksSessionHandlerPlugin::<Sendables>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(
        SteamworksServer::<Sendables>::new(&client)
            .expect("Couldn't create steamworks server"),
    );
}
```

### Client

This setup is very simple, all you need to do is add the `SteamworksSessionHandlerPlugin::<Sendables>` plugin and call the `SteamworksClient::<TSendables>::create(&Client, SteamId, &mut Commands)` function.
This creates an entity with both the `SteamworksSessionHandler::<Sendables>` and `Session::<TSendables>` component.

```rust
fn main() {
    App::new()
        .add_plugins((
            // Other plugins...
            SteamworksSessionHandlerPlugin::<Sendables>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, client: Res<Client>) {
    SteamworksClient::<Sendables>::create(
        &client,
        steam_id_to_connect_to,
        &mut commands,
    ).expect("FAIL");
}
```