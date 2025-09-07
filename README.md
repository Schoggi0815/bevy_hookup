# BEVY HOOKUP

A simple library for syncing bevy components over arbitrary sessions.

This repo consists of multiple crates:
- `bevy_hookup_core`: The core part of the syncing logic, not very useful without a session implementation.
- `bevy_hookup_messenger_self`: The simplest implementation of a session, which just hooks into the same world. This can be used for singleplayer implementations. With this implementation the Entity IDs will be changed for the shared entities, so you don't have collisions of Entity IDs with the shared components.
- `bevy_hookup_messenger_websocket`: A websocket implementation for the session. It has a server and client part, but they are only different when building the connection, the sharing of entities and components works in both directions the same way.

## Examples

For sharing a component you need a struct that implements the `SendableComponent<TSendables>` trait. You will also need to define the `TSendables` type, this is used to Serialize the shared components to one common type, this makes it easy for serialization and deserialization.

The following example shows how to implement a `SyncName` component which syncs the name of entites.
This works over all sessions, for simplicity the `SelfSession` is used here.

---

`sendables.rs`
```rust
#[derive(Clone, Serialize, Deserialize)]
pub enum Sendables {
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

impl SendableComponent<Sendables> for SyncName {
    fn to_sendable(&self) -> Sendables {
        Sendables::SyncName(self.clone())
    }

    fn from_sendable(sendable: Sendables) -> Option<Self> {
        match sendable {
            Sendables::SyncName(sync_name) => Some(sync_name),
            _ => None,
        }
    }
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