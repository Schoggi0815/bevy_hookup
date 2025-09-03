use bevy::prelude::*;
use bevy_hookup_core::{
    hook_session::SessionMessenger, hookup_component_plugin::HookupComponentPlugin,
    hookup_sendable_plugin::HookupSendablePlugin, owner_component::Owner, shared::Shared,
    sync_entity::SyncEntityOwner,
};
use bevy_hookup_messenger_self::self_session::SelfSession;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::{
    all_sendables::Sendables, test_component::TestComponent, test_component_2::TestComponent2,
};

mod all_sendables;
mod test_component;
mod test_component_2;

fn main() {
    App::new()
        .register_type::<Owner<TestComponent>>()
        .register_type::<Shared<TestComponent>>()
        .register_type::<Owner<TestComponent2>>()
        .register_type::<Shared<TestComponent2>>()
        .add_plugins((
            DefaultPlugins,
            HookupSendablePlugin::<Sendables>::default(),
            HookupComponentPlugin::<Sendables, TestComponent>::default(),
            HookupComponentPlugin::<Sendables, TestComponent2>::default(),
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_entity)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(SelfSession::<Sendables>::new().to_session());

    commands.spawn(Camera3d::default());
}

fn spawn_entity(mut commands: Commands, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        commands.spawn((
            SyncEntityOwner::new(),
            Owner::new(TestComponent { test_field: 2 }),
            Owner::new(TestComponent2 { test_field: 4 }),
        ));
    }
}
