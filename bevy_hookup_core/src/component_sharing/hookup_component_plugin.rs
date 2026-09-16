use std::marker::PhantomData;

use bevy::{ecs::component::Mutable, prelude::*};
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    client_id::ClientId,
    component_sharing::{
        component_origin::ComponentOrigin, component_type_id::ComponentTypeId,
        receive_component_systems::ReceiveComponentSystems,
        send_component_systems::SendComponentSystems, share_component::ShareComponent,
    },
    connection::{
        Connection,
        connection_id::ConnectionId,
        remote_action::{ComponentAction, RemoteAction},
    },
    entity_sharing::{
        receive_entity_systems::ReceiveEntitySystems,
        send_entity_systems::SendEntitySystems,
        sync_entity::{SyncEntity, SyncEntityOwner},
    },
    event_sharing::session_events::{
        SessionAddedComponent, SessionRemovedComponent, SessionUpdatedComponent,
    },
};

pub struct HookupReflectComponentPlugin<
    TComponent: Component<Mutability = Mutable> + Serialize + DeserializeOwned + TypePath + Reflect,
    const COMPONENT_ID: u64,
>(PhantomData<TComponent>);

impl<
    TComponent: Component<Mutability = Mutable> + Serialize + DeserializeOwned + TypePath + Reflect,
    const COMPONENT_ID: u64,
> Default for HookupReflectComponentPlugin<TComponent, COMPONENT_ID>
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<
    TComponent: Component<Mutability = Mutable> + Serialize + DeserializeOwned + TypePath + Reflect,
    const COMPONENT_ID: u64,
> Plugin for HookupReflectComponentPlugin<TComponent, COMPONENT_ID>
{
    fn build(&self, app: &mut App) {
        app.add_plugins((
            HookupComponentPlugin::<TComponent, COMPONENT_ID>::default(),
            HookupComponentTypePlugin::<TComponent>::default(),
        ));
    }
}

pub struct HookupComponentTypePlugin<
    TComponent: Component<Mutability = Mutable> + TypePath + Reflect,
>(PhantomData<TComponent>);

impl<TComponent: Component<Mutability = Mutable> + TypePath + Reflect> Default
    for HookupComponentTypePlugin<TComponent>
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<TComponent: Component<Mutability = Mutable> + TypePath + Reflect> Plugin
    for HookupComponentTypePlugin<TComponent>
{
    fn build(&self, app: &mut App) {
        app.register_type::<ComponentOrigin<TComponent, ConnectionId>>()
            .register_type::<ComponentOrigin<TComponent, ClientId>>()
            .register_type::<ShareComponent<TComponent>>();
    }
}

pub struct HookupComponentPlugin<
    TComponent: Component<Mutability = Mutable> + Serialize + DeserializeOwned,
    const COMPONENT_ID: u64,
>(PhantomData<TComponent>);

impl<
    TComponent: Component<Mutability = Mutable> + Serialize + DeserializeOwned,
    const COMPONENT_ID: u64,
> Default for HookupComponentPlugin<TComponent, COMPONENT_ID>
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<
    TComponent: Component<Mutability = Mutable> + Serialize + DeserializeOwned,
    const COMPONENT_ID: u64,
> Plugin for HookupComponentPlugin<TComponent, COMPONENT_ID>
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
            ReceiveComponentSystems::<TComponent>::default().after(ReceiveEntitySystems),
        )
        .configure_sets(
            FixedUpdate,
            SendComponentSystems::<TComponent>::default().after(SendEntitySystems),
        );
    }
}

impl<
    TComponent: Component<Mutability = Mutable> + Serialize + DeserializeOwned,
    const COMPONENT_ID: u64,
> HookupComponentPlugin<TComponent, COMPONENT_ID>
{
    fn send_removed_owned(
        trigger: On<Remove, ShareComponent<TComponent>>,
        sync_entities: Query<(
            &SyncEntity,
            &ShareComponent<TComponent>,
            Option<&SyncEntityOwner>,
        )>,
        connections: Query<&mut Connection>,
    ) -> Result {
        let Ok((removed_entity, removed_owner, removed_entity_owner)) =
            sync_entities.get(trigger.entity)
        else {
            warn!("Removed Owner not found!");
            return Ok(());
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

            session.component_removed(removed_entity.sync_id, ComponentTypeId(COMPONENT_ID))?;
        }

        Ok(())
    }

    pub fn send_owned(
        owned_components: Query<(
            &mut ShareComponent<TComponent>,
            Ref<TComponent>,
            &SyncEntity,
            Option<Ref<SyncEntityOwner>>,
        )>,
        mut connections: Query<&mut Connection>,
    ) -> Result {
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
                    session.component_added(
                        sync_entity.sync_id,
                        ComponentTypeId(COMPONENT_ID),
                        component.into_inner(),
                    )?;
                    share_component
                        .on_sessions
                        .push(session.get_connection_id());
                } else if is_allowed && is_on {
                    session.componend_updated(
                        sync_entity.sync_id,
                        ComponentTypeId(COMPONENT_ID),
                        component.into_inner(),
                    )?;
                } else if is_on && !is_allowed {
                    if is_entity_allowed {
                        session.component_removed(
                            sync_entity.sync_id,
                            ComponentTypeId(COMPONENT_ID),
                        )?;
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

        Ok(())
    }

    fn check_session_channels(
        connections: Query<&Connection>,
        mut sync_entites: Query<(
            &SyncEntity,
            Entity,
            Option<&SyncEntityOwner>,
            Option<&mut TComponent>,
        )>,
        mut commands: Commands,
    ) {
        for connection in connections {
            for (action, entity_id) in connection.messages().filter_map(|ra| match ra {
                RemoteAction::Component {
                    action,
                    entity_id,
                    component_type_id,
                } => {
                    if component_type_id.0 != COMPONENT_ID {
                        return None;
                    }

                    Some((action, entity_id))
                }
                _ => None,
            }) {
                let Some((_, entity, owner, data)) = sync_entites
                    .iter_mut()
                    .find(|(se, ..)| se.sync_id == *entity_id)
                else {
                    warn!("Entity for component event not found");
                    continue;
                };

                if let Some(owner) = owner
                    && !owner
                        .session_write_filter
                        .is_allowed(&connection.get_connection_id())
                {
                    warn!(
                        "Connection [{:?}] tried to modify unallowed entity [{:?}]!",
                        connection.get_connection_id(),
                        entity_id
                    );
                    continue;
                }

                match action {
                    ComponentAction::Remove => {
                        commands
                            .entity(entity)
                            .remove::<TComponent>()
                            .remove::<ComponentOrigin<TComponent, ConnectionId>>()
                            .remove::<ComponentOrigin<TComponent, ClientId>>();
                        commands.trigger(SessionRemovedComponent::<TComponent> {
                            entity,
                            connection_id: connection.get_connection_id(),
                            phantom: default(),
                        });
                    }
                    ComponentAction::AddOrUpdate { component_data_raw } => {
                        let Some(component_data) = connection.get_data(component_data_raw) else {
                            continue;
                        };

                        let Some(mut data) = data else {
                            commands.entity(entity).insert(component_data).insert(
                                ComponentOrigin::<TComponent, _>::new(
                                    connection.get_connection_id(),
                                ),
                            );
                            commands.trigger(SessionAddedComponent::<TComponent> {
                                entity,
                                connection_id: connection.get_connection_id(),
                                phantom: default(),
                            });
                            continue;
                        };

                        *data = component_data;
                        commands.trigger(SessionUpdatedComponent::<TComponent> {
                            entity,
                            connection_id: connection.get_connection_id(),
                            phantom: default(),
                        });
                    }
                }
            }
        }
    }
}
