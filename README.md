# BEVY HOOKUP

A simple library for syncing bevy components over arbitrary sessions.

This repo consists of multiple crates:
- `bevy_hookup_core`: The core part of the syncing logic, not very useful without a session implementation.
- `bevy_hookup_macros`: This crate provides some helper macros.
- `bevy_hookup_messenger_websocket`: A websocket implementation for the session. It has a server and client part, but they are only different when building the connection, the sharing of entities and components works in both directions the same way.

## Examples

The `Sendable` type needs to implement `From<&TComponent>` and `Into<Option<TComponent>>` for every component you want to send.
The `bevy_hookup_macros` crate provides a simple macro to do this for single-field enum entries, which implement clone, just derive `Sendable` for the enum and add the `sendable` attribute to every field you want this to be implemented.
You can read more about how this macro works [here](bevy_hookup_macros/README.md).

The following example shows how to implement the sharing of the `Name` component.
Any component that implements `Serialize` and `Deserialize` from `serde` can be used for this example.
This works over all sessions.

---

`sendables.rs`
```rust
// The sendable macro is from the `bevy_hookup_macros` crate.
#[derive(Clone, Sendable, Serialize, Deserialize)]
pub enum Sendables {
    #[sendable]
    Name(Name),
    // more components would be added here...
}
```

---

`main.rs`
```rust
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            HookupSendablePlugin::<Sendables>::default(),
            HookupComponentPlugin::<Sendables, Name>::default(),
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (update_owner_name, update_shared_name))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3d::default());

    // Spawn an owned entity with a name
    commands.spawn((
        SyncEntityOwner::new(),
        Name::new("Hello"),
        ShareComponent::<Name>::default(),
    ));
}
```

To learn how to set up a websocket session, check out the readme for the `bevy_hookup_messenger_websocket` crate [here](bevy_hookup_messenger_websocket/README.md).

You can also check out and test the basic example [here](examples/basic/src/main.rs).

## Filtering

If you want to share an entity or component with only some Sessions and not with everyone, you can use `SessionFilters`.

There are 5 types of session filters:

- AllowAll
    - This allows every session.
- AllowNone
    - This allows no sessions.
- Blacklist
    - This prohibits a list of sessions.
- Whitelist
    - This allows a list of session.
- BlacklistReshare
    - Same as `Blacklist`, used internally for resharing. Don't use this, use `Blacklist` instead.

If you change the filters whilst an entity or component is already shared to another session, it will just send the appropriate remove or add action to the affected sessions.

The `ShareComponent` component has a read filter which can be set to determine which sessions this component gets sent to.

The default value for this is `AllowAll`.

```rust
commands.spawn((
    SyncEntityOwner::new(),
    Name::new("Hello"),
    ShareComponent::<Name>::default()
        .with_read_filter(SessionFilter::Whitelist(vec![some_session_id])),
));
```

The `SyncEntityOwner` has both a read and a write filter. Other sessions can add their own `ShareComponent` components to a shared entity, sending over another component, if the write filter of the owner allows it.

The read filter works the same was as for components. If you don't allow a session to read an entity, it will also hide all components of this entity, there's no need to define the read filter on both components.

The default value for the read filter is `AllowAll` and for the write filter it is `AllowNone`.

## Resharing

If you want to use a client-authorative design, you need a way for the client to share its components to other clients connected to the same server.
The easiest way to do this is by using the `ReshareComponentPlugin` and the `ReshareEntityPlugin`.

To add resharing to the example above, you just need to update the used plugins to the following:

```diff
        .add_plugins((
            DefaultPlugins,
            HookupSendablePlugin::<Sendables>::default(),
            HookupComponentPlugin::<Sendables, Name>::default(),
+           ReshareComponentPlugin::<Name>::default(),
+           ReshareEntityPlugin::<Sendables>::default(),
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
        ))
```

Now when the client creates its own entity, that is shared to the server, the server will mirror the sharing with a filter for everyone to see, except for the original sender.