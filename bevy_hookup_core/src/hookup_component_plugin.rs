use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{
    external_component::ExternalComponent,
    owner_component::Owner,
    receive_component_set::ReceiveComponentSet,
    send_component_set::SendComponentSet,
    session::Session,
    session_action::SessionAction,
    shared::Shared,
    sync_entity::{SyncEntity, SyncEntityOwner},
};

pub struct HookupComponentPlugin<
    TSendables: Send + Sync + 'static + Clone + for<'a> From<&'a TComponent> + Into<Option<TComponent>>,
    TComponent: Send + Sync + 'static,
> {
    _phantom: PhantomData<TSendables>,
    _phantom_component: PhantomData<TComponent>,
}

impl<
    TSendables: Send + Sync + 'static + Clone + for<'a> From<&'a TComponent> + Into<Option<TComponent>>,
    TComponent: Send + Sync + 'static,
> Default for HookupComponentPlugin<TSendables, TComponent>
{
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
            _phantom_component: Default::default(),
        }
    }
}

impl<
    TSendables: 'static + Send + Sync + Clone + for<'a> From<&'a TComponent> + Into<Option<TComponent>>,
    TComponent: 'static + Send + Sync,
> Plugin for HookupComponentPlugin<TSendables, TComponent>
{
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            FixedUpdate,
            (
                send_owned::<TSendables, TComponent>
                    .in_set(SendComponentSet::<TComponent>::default()),
                check_session_channels::<TSendables, TComponent>
                    .in_set(ReceiveComponentSet::<TComponent>::default()),
            ),
        )
        .add_observer(send_removed_owned::<TSendables, TComponent>);
    }
}

fn send_removed_owned<
    TSendables: Send + Sync + 'static + Clone + for<'a> From<&'a TComponent> + Into<Option<TComponent>>,
    TComponent: Send + Sync + 'static,
>(
    trigger: On<Remove, Owner<TComponent>>,
    sync_entities: Query<(&SyncEntity, &Owner<TComponent>, Option<&SyncEntityOwner>)>,
    sessions: Query<&mut Session<TSendables>>,
) {
    let Ok((removed_entity, removed_owner, removed_entity_owner)) =
        sync_entities.get(trigger.entity)
    else {
        warn!("Removed Owner not found!");
        return;
    };

    let external_component =
        ExternalComponent::new(removed_entity.sync_id, removed_owner.component_id);

    for mut session in sessions {
        if let Some(removed_entity_owner) = removed_entity_owner
            && !removed_entity_owner
                .session_read_filter
                .allow_session(&session.get_session_id())
        {
            continue;
        }

        if !removed_owner
            .session_read_filter
            .allow_session(&session.get_session_id())
        {
            continue;
        }

        session.component_removed(external_component);
    }
}

pub fn send_owned<
    TSendables: Send + Sync + 'static + Clone + for<'a> From<&'a TComponent> + Into<Option<TComponent>>,
    TComponent: Send + Sync + 'static,
>(
    owned_components: Query<(
        &mut Owner<TComponent>,
        &SyncEntity,
        Option<Ref<SyncEntityOwner>>,
    )>,
    mut sessions: Query<&mut Session<TSendables>>,
) {
    for (mut owned_component, sync_entity, sync_owner) in owned_components {
        let component_changed = owned_component.is_changed();
        let sync_owner_changed = if let Some(ref sync_owner) = sync_owner {
            sync_owner.is_changed()
        } else {
            false
        };

        if !component_changed && !sync_owner_changed {
            continue;
        }

        let external_component =
            ExternalComponent::new(sync_entity.sync_id, owned_component.component_id);

        let sendable = TSendables::from(owned_component.get_inner());
        let session_filter = owned_component.session_read_filter.clone();

        for mut session in sessions.iter_mut() {
            let is_component_allowed = session_filter.allow_session(&session.get_session_id());
            let is_on = owned_component
                .on_sessions
                .contains(&session.get_session_id());

            let is_entity_allowed = if let Some(ref sync_owner) = sync_owner
                && !sync_owner
                    .session_read_filter
                    .allow_session(&session.get_session_id())
            {
                false
            } else {
                true
            };

            let is_allowed = is_component_allowed && is_entity_allowed;

            if is_allowed && !is_on {
                session.component_added(external_component, sendable.clone());
                owned_component.on_sessions.push(session.get_session_id());
            } else if is_allowed && is_on {
                session.componend_updated(external_component, sendable.clone());
            } else if is_on && !is_allowed {
                if is_entity_allowed {
                    session.component_removed(external_component);
                }
                owned_component.on_sessions = owned_component
                    .on_sessions
                    .clone()
                    .into_iter()
                    .filter(|sid| *sid != session.get_session_id())
                    .collect();
            }
        }
    }
}

fn check_session_channels<
    TSendables: Send + Sync + 'static + Clone + for<'a> From<&'a TComponent> + Into<Option<TComponent>>,
    TComponent: Send + Sync + 'static,
>(
    sessions: Query<&Session<TSendables>>,
    sync_entites: Query<(Entity, &SyncEntity, Option<&SyncEntityOwner>)>,
    mut shared_components: Query<(Entity, &mut Shared<TComponent>)>,
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
                    let Some(sended_component) =
                        Into::<Option<TComponent>>::into(component_data.clone())
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
                    let Some(sended_component) =
                        Into::<Option<TComponent>>::into(component_data.clone())
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

                    shared_component.update_inner(sended_component);
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
