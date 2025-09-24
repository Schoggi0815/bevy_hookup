# BEVY HOOKUP

A simple library for syncing bevy components over arbitrary sessions.

This repo consists of multiple crates:
- `bevy_hookup_core`: The core part of the syncing logic, not very useful without a session implementation.
- `bevy_hookup_macros`: This crate provides some helper macros.
- `bevy_hookup_messenger_self`: The simplest implementation of a session, which just hooks into the same world. This can be used for singleplayer implementations. With this implementation the Entity IDs will be changed for the shared entities, so you don't have collisions of Entity IDs with the shared components.
- `bevy_hookup_messenger_websocket`: A websocket implementation for the session. It has a server and client part, but they are only different when building the connection, the sharing of entities and components works in both directions the same way.

## Examples

The `Sendable` type needs to implement `From<&TComponent>` and `Into<Option<TComponent>>` for every component you want to send.
The `bevy_hookup_macros` crate provides a simple macro to do this for single-field enum entries, which implement clone, just derive `Sendable` for the enum and add the `sendable` attribute to every field you want this to be implemented.

The following example shows how to implement a `SyncName` component which syncs the name of entites.
This works over all sessions, for simplicity the `SelfSession` is used here.

---

`sendables.rs`
```rust
// The sendable macro is from the `bevy_hookup_macros` crate.
#[derive(Clone, Sendable, Serialize, Deserialize)]
pub enum Sendables {
    #[sendable]
    SyncName(SyncName),
    // more components would be added here...
}
```

---

`sync_name.rs`
```rust
// The derive of reflect is optional, makes it useful for inspecting the component in the bevy_inspector
#[derive(Default, Clone, Reflect, Serialize, Deserialize)]
pub struct SyncName {
    pub name: String,
}
```

---

`main.rs`
```rust
fn main() {
    App::new()
        // The calls of `register_type` are optional and only needed for the bevy_inspector
        .register_type::<Owner<SyncName>>()
        .register_type::<Shared<SyncName>>()
        .add_plugins((
            DefaultPlugins,
            HookupSendablePlugin::<Sendables>::default(),
            HookupComponentPlugin::<Sendables, SyncName>::default(),
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (update_owner_name, update_shared_name))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3d::default());

    // Initiate a simple `SelfSession`
    commands.spawn(SelfSession::<Sendables>::new().to_session());

    // Spawn an owned entity with a name
    commands.spawn((
        SyncEntityOwner::new(),
        Name::new("Hello"),
    ));
}

// This system inserts the `Owner<SyncName>` component for your owned entities.
fn update_owner_name(
    mut commands: Commands,
    names: Query<(Entity, &Name), (Changed<Name>, With<SyncEntityOwner>)>,
) {
    for (entity, name) in names {
        commands.entity(entity).insert(Owner::new(SyncName {
            name: name.as_str().to_string(),
        }));
    }
}

// This systems sets the Name component from the `Shared<SyncName>` components.
fn update_shared_name(
    mut commands: Commands,
    names: Query<(Entity, &Shared<SyncName>), Changed<Shared<SyncName>>>,
) {
    for (entity, shared_name) in names {
        commands
            .entity(entity)
            .insert(Name::new(shared_name.name.clone()));
    }
}
```