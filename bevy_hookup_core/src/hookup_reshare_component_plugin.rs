use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{
    external_component::ExternalComponent,
    from_session::FromSession,
    hookup_component_plugin::send_owned,
    owner_component::Owner,
    sendable_component::SendableComponent,
    session::Session,
    session_action::SessionAction,
    shared::Shared,
    sync_entity::{SyncEntity, SyncEntityOwner},
};

pub struct HookupReshareComponentPlugin<
    TSendables: Send + Sync + 'static + Clone,
    TComponent: SendableComponent<TSendables> + Send + Sync + 'static + PartialEq,
>(PhantomData<TSendables>, PhantomData<TComponent>);

impl<
    TSendables: Send + Sync + 'static + Clone,
    TComponent: SendableComponent<TSendables> + Send + Sync + 'static + PartialEq,
> Default for HookupReshareComponentPlugin<TSendables, TComponent>
{
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}

impl<
    TSendables: 'static + Send + Sync + Clone,
    TComponent: SendableComponent<TSendables> + 'static + Send + Sync + PartialEq,
> Plugin for HookupReshareComponentPlugin<TSendables, TComponent>
{
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            FixedUpdate,
            (
                send_owned::<TSendables, TComponent>,
                send_changes_shared::<TSendables, TComponent>,
                check_session_channels::<TSendables, TComponent>,
            ),
        );
    }
}

fn send_changes_shared<
    TSendables: Send + Sync + 'static + Clone,
    TComponent: SendableComponent<TSendables> + Send + Sync + 'static + PartialEq,
>(
    shared_components: Query<
        (&Shared<TComponent>, &SyncEntity, &FromSession),
        Changed<Shared<TComponent>>,
    >,
    mut sessions: Query<&mut Session<TSendables>>,
) {
    for (shared, sync, from_session) in shared_components {
        let Some(mut session) = sessions
            .iter_mut()
            .find(|session| session.get_session_id() == from_session.session_id)
        else {
            continue;
        };

        let external_component = ExternalComponent::new(sync.sync_id, shared.component_id);

        session.component_shared_updated(external_component, shared.inner.to_sendable());
    }
}

fn check_session_channels<
    TSendables: Send + Sync + 'static + Clone,
    TComponent: SendableComponent<TSendables> + Send + Sync + 'static + PartialEq,
>(
    sessions: Query<&Session<TSendables>>,
    sync_entites: Query<(Entity, &SyncEntity, Option<&SyncEntityOwner>)>,
    mut shared_components: Query<(Entity, &mut Shared<TComponent>)>,
    mut owned_components: Query<&mut Owner<TComponent>>,
    mut commands: Commands,
) {
    for session in sessions {
        let mut unused_actions = Vec::new();
        for session_action in session.channels.receiver.try_iter() {
            match session_action {
                SessionAction::AddComponent {
                    ref component_data,
                    ref external_component,
                } => {
                    let Some(sended_component) = TComponent::from_sendable(component_data.clone())
                    else {
                        unused_actions.push(session_action);
                        continue;
                    };

                    let Some((sync_entity, _, owner)) = sync_entites
                        .iter()
                        .find(|se| se.1.sync_id == external_component.entity_id)
                    else {
                        continue;
                    };

                    if let Some(owner) = owner
                        && !owner
                            .session_write_filter
                            .allow_session(&session.get_session_id())
                    {
                        warn!(
                            "Session [{:?}] tried to add to unallowed entity!",
                            session.get_session_id()
                        );
                        continue;
                    }

                    commands.entity(sync_entity).insert(Shared::new(
                        sended_component,
                        external_component.component_id,
                    ));
                }
                SessionAction::UpdateComponent {
                    ref component_data,
                    ref external_component,
                } => {
                    let Some(sended_component) = TComponent::from_sendable(component_data.clone())
                    else {
                        unused_actions.push(session_action);
                        continue;
                    };

                    let Some((_, mut shared_component)) = shared_components
                        .iter_mut()
                        .find(|c| c.1.component_id == external_component.component_id)
                    else {
                        continue;
                    };

                    if sended_component == shared_component.inner {
                        continue;
                    }

                    shared_component.update_inner(sended_component);
                }
                SessionAction::UpdateSharedComponent {
                    ref component_data,
                    ref external_component,
                } => {
                    let Some(sended_component) = TComponent::from_sendable(component_data.clone())
                    else {
                        unused_actions.push(session_action);
                        continue;
                    };

                    let Some(mut owned_component) = owned_components
                        .iter_mut()
                        .find(|c| c.component_id == external_component.component_id)
                    else {
                        continue;
                    };

                    if !owned_component
                        .session_write_filter
                        .allow_session(&session.get_session_id())
                    {
                        warn!(
                            "Session [{:?}] tried to update unallowed component!",
                            session.get_session_id()
                        );
                        continue;
                    }

                    if sended_component == owned_component.inner {
                        continue;
                    }

                    owned_component.update_inner(sended_component);
                }
                SessionAction::RemoveComponent { external_component } => {
                    let Some((sync_entity, _)) = shared_components
                        .iter()
                        .find(|sc| sc.1.component_id == external_component.component_id)
                    else {
                        unused_actions.push(session_action);
                        continue;
                    };

                    commands.entity(sync_entity).remove::<Shared<TComponent>>();
                }
                _ => {
                    unused_actions.push(session_action);
                }
            }
        }
        unused_actions
            .into_iter()
            .for_each(|sa| session.channels.sender.try_send(sa).expect("Unbounded"));
    }
}
