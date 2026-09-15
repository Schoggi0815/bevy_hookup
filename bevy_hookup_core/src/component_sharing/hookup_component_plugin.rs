use std::marker::PhantomData;

use bevy::{ecs::component::Mutable, prelude::*};

use crate::{
    component_sharing::{
        receive_component_systems::ReceiveComponentSystems,
        send_component_systems::SendComponentSystems, share_component::ShareComponent,
    },
    connection::{Connection, remote_action::RemoteAction},
    entity_sharing::{
        receive_entity_systems::ReceiveEntitySystems,
        sync_entity::{SyncEntity, SyncEntityOwner},
    },
    event_sharing::session_events::{
        SessionAddedComponent, SessionRemovedComponent, SessionUpdatedComponent,
    },
};

pub struct HookupComponentPlugin<
    TSendables: Send + Sync + 'static + Clone + for<'a> From<&'a TComponent> + Into<Option<TComponent>>,
    TComponent: Send + Sync + 'static + Component<Mutability = Mutable>,
> {
    _phantom: PhantomData<TSendables>,
    _phantom_component: PhantomData<TComponent>,
}

impl<
    TSendables: Send + Sync + 'static + Clone + for<'a> From<&'a TComponent> + Into<Option<TComponent>>,
    TComponent: Send + Sync + 'static + Component<Mutability = Mutable>,
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
    TComponent: 'static + Send + Sync + Component<Mutability = Mutable>,
> Plugin for HookupComponentPlugin<TSendables, TComponent>
{
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            FixedUpdate,
            (
                Self::send_owned.in_set(SendComponentSystems::<TComponent>::default()),
                Self::check_session_channels
                    .in_set(ReceiveComponentSystems::<TComponent>::default()),
            ),
        )
        .add_observer(Self::send_removed_owned)
        .configure_sets(
            FixedUpdate,
            ReceiveComponentSystems::<TComponent>::default()
                .after(ReceiveEntitySystems::<TSendables>::default()),
        );
    }
}

impl<
    TSendables: 'static + Send + Sync + Clone + for<'a> From<&'a TComponent> + Into<Option<TComponent>>,
    TComponent: 'static + Send + Sync + Component<Mutability = Mutable>,
> HookupComponentPlugin<TSendables, TComponent>
{
    fn send_removed_owned(
        trigger: On<Remove, ShareComponent<TComponent>>,
        sync_entities: Query<(
            &SyncEntity,
            &ShareComponent<TComponent>,
            Option<&SyncEntityOwner>,
        )>,
        connections: Query<&mut Connection<TSendables>>,
    ) {
        let Ok((removed_entity, removed_owner, removed_entity_owner)) =
            sync_entities.get(trigger.entity)
        else {
            warn!("Removed Owner not found!");
            return;
        };

        for mut session in connections {
            if let Some(removed_entity_owner) = removed_entity_owner
                && !removed_entity_owner
                    .session_read_filter
                    .is_allowed(&session.get_connection_id())
            {
                continue;
            }

            if !removed_owner
                .read_filter
                .is_allowed(&session.get_connection_id())
            {
                continue;
            }

            session.component_removed(removed_entity.sync_id);
        }
    }

    pub fn send_owned(
        owned_components: Query<(
            &mut ShareComponent<TComponent>,
            Ref<TComponent>,
            &SyncEntity,
            Option<Ref<SyncEntityOwner>>,
        )>,
        mut connections: Query<&mut Connection<TSendables>>,
    ) {
        for (mut share_component, component, sync_entity, sync_owner) in owned_components {
            let component_changed = component.is_changed();
            let share_changed = share_component.is_changed();
            let sync_owner_changed = if let Some(ref sync_owner) = sync_owner {
                sync_owner.is_changed()
            } else {
                false
            };

            if !component_changed && !sync_owner_changed && !share_changed {
                continue;
            }

            let sendable = TSendables::from(component.into_inner());
            let session_filter = share_component.read_filter.clone();

            for mut session in connections.iter_mut() {
                let is_component_allowed = session_filter.is_allowed(&session.get_connection_id());
                let is_on = share_component
                    .on_sessions
                    .contains(&session.get_connection_id());

                let is_entity_allowed = if let Some(ref sync_owner) = sync_owner
                    && !sync_owner
                        .session_read_filter
                        .is_allowed(&session.get_connection_id())
                {
                    false
                } else {
                    true
                };

                let is_allowed = is_component_allowed && is_entity_allowed;

                if is_allowed && !is_on {
                    session.component_added(sync_entity.sync_id, sendable.clone());
                    share_component
                        .on_sessions
                        .push(session.get_connection_id());
                } else if is_allowed && is_on {
                    session.componend_updated(sync_entity.sync_id, sendable.clone());
                } else if is_on && !is_allowed {
                    if is_entity_allowed {
                        session.component_removed(sync_entity.sync_id);
                    }
                    share_component.on_sessions = share_component
                        .on_sessions
                        .clone()
                        .into_iter()
                        .filter(|sid| *sid != session.get_connection_id())
                        .collect();
                }
            }
        }
    }

    fn check_session_channels(
        sessions: Query<&Connection<TSendables>>,
        mut sync_entites: Query<(
            &SyncEntity,
            Entity,
            Option<&SyncEntityOwner>,
            Option<&mut TComponent>,
        )>,
        mut commands: Commands,
    ) {
        for session in sessions {
            let mut unused_actions = Vec::new();
            for session_action in session.channels.receiver.try_iter() {
                match session_action {
                    RemoteAction::AddComponent {
                        ref component_data,
                        ref entity_id,
                    } => {
                        let Some(sended_component) =
                            Into::<Option<TComponent>>::into(component_data.clone())
                        else {
                            unused_actions.push(session_action);
                            continue;
                        };

                        let Some((_, entity, owner, _)) = sync_entites
                            .iter()
                            .find(|(se, ..)| se.sync_id == *entity_id)
                        else {
                            continue;
                        };

                        if let Some(owner) = owner
                            && !owner
                                .session_write_filter
                                .is_allowed(&session.get_connection_id())
                        {
                            warn!(
                                "Session [{:?}] tried to add to unallowed entity!",
                                session.get_connection_id()
                            );
                            continue;
                        }

                        commands.entity(entity).insert(sended_component);
                        commands.trigger(SessionAddedComponent::<TComponent> {
                            entity,
                            connection_id: session.get_connection_id(),
                            phantom: default(),
                        });
                    }
                    RemoteAction::UpdateComponent {
                        ref component_data,
                        ref entity_id,
                    } => {
                        let Some(sended_component) =
                            Into::<Option<TComponent>>::into(component_data.clone())
                        else {
                            unused_actions.push(session_action);
                            continue;
                        };

                        let Some((_, entity, _, data)) = sync_entites
                            .iter_mut()
                            .find(|(sync_entity, ..)| sync_entity.sync_id == *entity_id)
                        else {
                            continue;
                        };

                        let Some(mut data) = data else {
                            continue;
                        };

                        *data = sended_component;
                        commands.trigger(SessionUpdatedComponent::<TComponent> {
                            entity,
                            connection_id: session.get_connection_id(),
                            phantom: default(),
                        });
                    }
                    RemoteAction::RemoveComponent { entity_id } => {
                        let Some((_, entity, ..)) = sync_entites
                            .iter()
                            .find(|(sync_entity, ..)| sync_entity.sync_id == entity_id)
                        else {
                            unused_actions.push(session_action);
                            continue;
                        };

                        commands.entity(entity).remove::<TComponent>();
                        commands.trigger(SessionRemovedComponent::<TComponent> {
                            entity,
                            connection_id: session.get_connection_id(),
                            phantom: default(),
                        });
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
}
