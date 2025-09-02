use bevy::prelude::*;
use bevy_hookup_core::{
    hookup_component_plugin::HookupComponentPlugin, hookup_sendable_plugin::HookupSendablePlugin,
    owner_component::Owner, session_handler::SessionHandler, shared_component::SharedComponent,
};
use bevy_hookup_messager_self::self_session::SelfSession;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::{all_sendables::Sendables, test_component::TestComponent};

mod all_sendables;
mod test_component;

fn main() {
    App::new()
        .register_type::<Owner<TestComponent>>()
        .register_type::<SharedComponent<TestComponent>>()
        .add_plugins((
            DefaultPlugins,
            HookupSendablePlugin::<Sendables>::default(),
            HookupComponentPlugin::<Sendables, TestComponent>::default(),
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut session_handler: ResMut<SessionHandler<Sendables>>) {
    session_handler.add_session(SelfSession::new);

    commands.spawn(Camera3d::default());

    commands.spawn((Owner::new(TestComponent { test_field: 2 }),));
}
