use bevy::prelude::*;
use bevy_hookup_core::{
    hook_session::SessionMessenger, hookup_component_plugin::HookupComponentPlugin,
    hookup_event_plugin::HookupEventPlugin, hookup_sendable_plugin::HookupSendablePlugin,
    owner_component::Owner, received_event::ReceivedEvent, send_event::SendEvent,
    session_filter::SessionFilter, shared::Shared, sync_entity::SyncEntityOwner,
};
use bevy_hookup_messenger_self::self_session::SelfSession;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::{
    all_sendables::Sendables, sync_name::SyncName, test_component::TestComponent,
    test_component_2::TestComponent2, test_event::TestEvent,
};

mod all_sendables;
mod sync_name;
mod test_component;
mod test_component_2;
mod test_event;

#[tokio::main]
async fn main() {
    App::new()
        .register_type::<Owner<TestComponent>>()
        .register_type::<Shared<TestComponent>>()
        .register_type::<Owner<TestComponent2>>()
        .register_type::<Shared<TestComponent2>>()
        .register_type::<Owner<SyncName>>()
        .register_type::<Shared<SyncName>>()
        .add_plugins((
            DefaultPlugins,
            HookupSendablePlugin::<Sendables>::default(),
            HookupComponentPlugin::<Sendables, TestComponent>::default(),
            HookupComponentPlugin::<Sendables, TestComponent2>::default(),
            HookupComponentPlugin::<Sendables, SyncName>::default(),
            HookupEventPlugin::<Sendables, TestEvent>::default(),
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (spawn_entity, spawn_session, send_event))
        .add_systems(Update, (update_owner_name, update_shared_name))
        .add_observer(receive_event)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3d::default());
}

fn spawn_entity(mut commands: Commands, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        commands.spawn((
            SyncEntityOwner::new(),
            Name::new("Hello"),
            Owner::new(TestComponent { test_field: 2 }),
            Owner::new(TestComponent2 { test_field: 4 })
                .with_read_filter(SessionFilter::Whitelist(Vec::new())),
        ));
    }
}

fn spawn_session(mut commands: Commands, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Enter) {
        commands.spawn(SelfSession::<Sendables>::new().to_session());
    }
}

fn send_event(mut commands: Commands, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::KeyE) {
        commands.trigger(SendEvent::new(TestEvent { test_value: 12 }));
    }
}

fn receive_event(event: On<ReceivedEvent<TestEvent>>) {
    info!("Received event: {:?}", event.event());
}

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
