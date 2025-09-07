# Websocket Messenger

A session implementation that works over a websocket channel. This includes both the listener and client for the websocket.

## Example

You will need to use tokio and make the main method of your app async.

### Server

All you need to do is register the `WebsocketServerPlugin::<Sendables>` plugin and add the `WebsocketServer::<Sendables>` resource.
You can add the resource anytime, this will crate the listener for clients to connect to.
Every connected client will get its own `Session` registered.

```rust
#[tokio::main]
async fn main() {
    App::new()
        .add_plugins((
            // Other plugins...
            WebsocketServerPlugin::<Sendables>::default(),
        ))
        .insert_resource(WebsocketServer::<Sendables>::new(
            // use ip with port here
            "0.0.0.0:1234".to_string(),
        ))
        .run();
}
```

### Client

This setup is very similar to the server, you just need to register the `WebsocketClientPlugin::<Sendables>` plugin and add the `WebsocketClient::<Sendables>` resource.
The resource can of course be added later on, as soon as you create it, it will try to connect to the server using the specified address and port. As soon as a connection is established, the corresponding `Session` is created.

```rust
#[tokio::main]
async fn main() {
    App::new()
        .add_plugins((
            // Other plugins...
            WebsocketClientPlugin::<Sendables>::default(),
        ))
        .insert_resource(WebsocketClient::<Sendables>::new(
            // use ip with port here
            "ws://123.123.123.123:1234".to_string(),
        ))
        .run();
}
```