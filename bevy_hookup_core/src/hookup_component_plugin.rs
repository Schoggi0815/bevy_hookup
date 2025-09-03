use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{
    external_component::ExternalComponent, owner_component::Owner,
    sendable_component::SendableComponent, session_handler::SessionHandler, shared::Shared,
    sync_entity::SyncEntity,
};

#[derive(Default)]
pub struct HookupComponentPlugin<
    TSendables: Send + Sync + 'static,
    TComponent: SendableComponent<TSendables> + Send + Sync + 'static,
> {
    _phantom: PhantomData<TSendables>,
    _phantom_component: PhantomData<TComponent>,
}

impl<
    TSendables: 'static + Send + Sync,
    TComponent: SendableComponent<TSendables> + 'static + Send + Sync,
> Plugin for HookupComponentPlugin<TSendables, TComponent>
{
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            FixedUpdate,
            (
                send_owned::<TSendables, TComponent>,
                check_session_channels::<TSendables, TComponent>,
            ),
        );
    }
}

fn send_owned<
    TSendables: Send + Sync + 'static,
    TComponent: SendableComponent<TSendables> + Send + Sync + 'static,
>(
    owned_components: Query<(Ref<Owner<TComponent>>, Entity, &SyncEntity)>,
    session_handler: Res<SessionHandler<TSendables>>,
    mut commands: Commands,
) {
    for (owned_component, owned_entity, sync_entity) in owned_components {
        let external_component =
            ExternalComponent::new(sync_entity.sync_id, owned_component.component_id);

        if owned_component.remove {
            for session in session_handler.get_sessions() {
                session.component_removed(external_component);
            }
            commands.entity(owned_entity).remove::<Owner<TComponent>>();
        }

        if owned_component.is_added() {
            let session_component = owned_component.into_inner().get_inner();
            for session in session_handler.get_sessions() {
                session.component_added(external_component, session_component.to_sendable());
            }
        } else if owned_component.is_changed() {
            let session_component = owned_component.into_inner().get_inner();
            for session in session_handler.get_sessions() {
                session.componend_updated(external_component, session_component.to_sendable());
            }
        }
    }
}

pub fn check_session_channels<
    TSendables: Send + Sync + 'static,
    TComponent: SendableComponent<TSendables> + Send + Sync + 'static,
>(
    session_handler: ResMut<SessionHandler<TSendables>>,
    sync_entites: Query<(Entity, &SyncEntity)>,
    mut shared_components: Query<(Entity, &mut Shared<TComponent>)>,
    mut commands: Commands,
) {
    for session in session_handler.get_sessions() {
        for added_message in session.channels.added.1.try_iter() {
            let Some(sended_component) = TComponent::from_sendable(added_message.component_data)
            else {
                continue;
            };

            let Some((sync_entity, _)) = sync_entites
                .iter()
                .find(|se| se.1.sync_id == added_message.external_component.entity_id)
            else {
                continue;
            };

            commands.entity(sync_entity).insert(Shared::new(
                sended_component,
                added_message.external_component.component_id,
            ));
        }
        for updated_message in session.channels.updated.1.try_iter() {
            let Some(sended_component) = TComponent::from_sendable(updated_message.component_data)
            else {
                continue;
            };

            let Some((_, mut shared_component)) = shared_components
                .iter_mut()
                .find(|c| c.1.component_id == updated_message.external_component.component_id)
            else {
                continue;
            };

            shared_component.update_inner(sended_component);
        }
        for removed_message in session.channels.removed.1.try_iter() {
            let Some((sync_entity, _)) = sync_entites
                .iter()
                .find(|se| se.1.sync_id == removed_message.external_component.entity_id)
            else {
                continue;
            };

            commands.entity(sync_entity).remove::<Shared<TComponent>>();
        }
    }
}
