use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{
    owner_component::Owner, sendable_component::SendableComponent, session_handler::SessionHandler,
    shared_component::SharedComponent,
};

#[derive(Default)]
pub struct HookupPlugin<
    TSendables: Send + Sync + 'static,
    TComponent: SendableComponent<TSendables> + Send + Sync + 'static,
> {
    _phantom: PhantomData<TSendables>,
    _phantom_component: PhantomData<TComponent>,
}

impl<
    TSendables: 'static + Send + Sync,
    TComponent: SendableComponent<TSendables> + 'static + Send + Sync,
> Plugin for HookupPlugin<TSendables, TComponent>
{
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(SessionHandler::<TSendables>::new())
            .add_systems(
                FixedUpdate,
                (
                    send_owned::<TSendables, TComponent>,
                    check_sessions::<TSendables, TComponent>,
                ),
            );
    }
}

fn send_owned<
    TSendables: Send + Sync + 'static,
    TComponent: SendableComponent<TSendables> + Send + Sync + 'static,
>(
    owned_tests: Query<(Entity, Ref<Owner<TComponent>>)>,
    session_handler: Res<SessionHandler<TSendables>>,
    mut removed_owneds: RemovedComponents<Owner<TComponent>>,
) {
    for (owned_entity, owned_test) in owned_tests {
        if owned_test.is_added() {
            let session_component = owned_test.into_inner().get_inner();
            for session in session_handler.get_sessions() {
                session.component_added(owned_entity, session_component.to_sendable());
            }
        } else if owned_test.is_changed() {
            let session_component = owned_test.into_inner().get_inner();
            for session in session_handler.get_sessions() {
                session.componend_updated(owned_entity, session_component.to_sendable());
            }
        }
    }

    for entity in removed_owneds.read() {
        for session in session_handler.get_sessions() {
            session.component_removed(entity);
        }
    }
}

pub fn check_sessions<
    TSendables: Send + Sync + 'static,
    TComponent: SendableComponent<TSendables> + Send + Sync + 'static,
>(
    session_handler: ResMut<SessionHandler<TSendables>>,
    mut shared_components: Query<(Entity, &mut SharedComponent<TComponent>)>,
    mut commands: Commands,
) {
    for session in session_handler.get_sessions() {
        for added_message in session.channels.added.1.try_iter() {
            let Some(sended_component) = TComponent::from_sendable(added_message.component_data)
            else {
                continue;
            };

            commands.spawn(SharedComponent::new(sended_component, added_message.entity));
        }
        for updated_message in session.channels.updated.1.try_iter() {
            let Some(sended_component) = TComponent::from_sendable(updated_message.component_data)
            else {
                continue;
            };

            let Some((_, mut shared_component)) = shared_components
                .iter_mut()
                .find(|c| c.1.external_entity == updated_message.entity)
            else {
                continue;
            };

            shared_component.update_inner(sended_component);
        }
        for removed_message in session.channels.removed.1.try_iter() {
            let Some((shared_entity, _)) = shared_components
                .iter_mut()
                .find(|c| c.1.external_entity == removed_message.entity)
            else {
                continue;
            };

            commands.entity(shared_entity).despawn();
        }
    }
}
