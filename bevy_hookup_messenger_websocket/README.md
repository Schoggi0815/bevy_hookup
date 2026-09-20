# Websocket Messenger

[![crates.io](https://img.shields.io/crates/v/bevy_hookup_messenger_websocket)](https://crates.io/crates/bevy_hookup_messenger_websocket)
[![docs.rs](https://docs.rs/bevy_hookup_messenger_websocket/badge.svg)](https://docs.rs/bevy_hookup_messenger_websocket)

A session implementation that works over a websocket channel. This includes both the listener and client for the websocket.

## Example

You will need to use tokio and make the main method of your app async.

### Server

All you need to do is register the `WebsocketServerPlugin` plugin and add an entity with the `WebsocketServer` component.
You can add the resource anytime, this will crate the listener for clients to connect to.
Every connected client will get its own `Connection` registered.

```rust
#[tokio::main]
async fn main() {
    App::new()
        .add_plugins((
            // Other plugins...
            WebsocketServerPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(WebsocketServer::new(
        // use ip with port here
        "0.0.0.0:1234".to_string(),
    ));
}
```

### Client

This setup is very similar to the server, you just need to register the `WebsocketClientPlugin` plugin and add an entity with the `WebsocketClient` component.
The resource can of course be added later on, as soon as you create it, it will try to connect to the server using the specified address and port. As soon as a connection is established, the corresponding `Connection` is created.

```rust
#[tokio::main]
async fn main() {
    App::new()
        .add_plugins((
            // Other plugins...
            WebsocketClientPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(WebsocketClient::new(
        // use ip with port here
        "ws://123.123.123.123:1234".to_string(),
    ));
}
```