use bevy::{ecs::system::NonSendMarker, prelude::*};
use bevy_hookup_core::{
    component_sharing::{
        hookup_component_plugin::HookupComponentPlugin, share_component::ShareComponent,
    },
    entity_sharing::sync_entity::SyncEntityOwner,
    event_sharing::{
        hookup_event_plugin::HookupEventPlugin, received_event::ReceivedEvent,
        send_event::SendEvent,
    },
    filter::Filter,
    hookup_sendable_plugin::HookupSendablePlugin,
    resharing::{
        reshare_component_plugin::ReshareComponentPlugin,
        reshare_entity_plugin::ReshareEntityPlugin,
    },
};
use bevy_hookup_messenger_websocket::{
    websocket_client::WebsocketClient, websocket_client_plugin::WebsocketClientPlugin,
    websocket_server::WebsocketServer, websocket_server_plugin::WebsocketServerPlugin,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::{
    test_component::TestComponent, test_component_2::TestComponent2, test_event::TestEvent,
};

mod test_component;
mod test_component_2;
mod test_event;

#[tokio::main]
async fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WebsocketClientPlugin,
            WebsocketServerPlugin,
            HookupSendablePlugin,
            HookupComponentPlugin::<TestComponent, 0>::default(),
            HookupComponentPlugin::<TestComponent2, 1>::default(),
            HookupComponentPlugin::<Name, 2>::default(),
            ReshareComponentPlugin::<Name>::default(),
            ReshareEntityPlugin,
            HookupEventPlugin::<TestEvent, 0>::default(),
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (spawn_entity, send_event, spawn_ws_server, spawn_ws_client),
        )
        .add_observer(receive_event)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3d::default());
}

fn spawn_ws_server(mut commands: Commands, input: Res<ButtonInput<KeyCode>>, _: NonSendMarker) {
    if input.just_pressed(KeyCode::F1) {
        commands.spawn(WebsocketServer::new_with_port(1526));
    }
}

fn spawn_ws_client(mut commands: Commands, input: Res<ButtonInput<KeyCode>>, _: NonSendMarker) {
    if input.just_pressed(KeyCode::F2) {
        commands.spawn(WebsocketClient::new_with_host_and_port(
            "127.0.0.1".into(),
            1526,
        ));
    }
}

fn spawn_entity(mut commands: Commands, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        commands.spawn((
            SyncEntityOwner::new(),
            Name::new("Hello"),
            ShareComponent::<Name>::default(),
            TestComponent { test_field: 2 },
            ShareComponent::<TestComponent>::default(),
            TestComponent2 { test_field: 4 },
            ShareComponent::<TestComponent2>::default()
                .with_read_filter(Filter::Whitelist(Vec::new())),
        ));
    }
}

fn send_event(mut commands: Commands, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::F3) {
        commands.trigger(SendEvent::new(TestEvent { test_value: 12 }));
    }
}

fn receive_event(event: On<ReceivedEvent<TestEvent>>) {
    info!("Received event: {:?}", event.event());
}
