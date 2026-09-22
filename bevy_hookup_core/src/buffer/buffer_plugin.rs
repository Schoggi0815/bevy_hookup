use std::marker::PhantomData;

use bevy::{
    ecs::{component::Mutable, entity_disabling::Disabled},
    prelude::*,
};
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    buffer::{
        buffer_object::BufferObject,
        buffer_systems::{RecieveBufferSystems, SendBufferSystems},
        component_buffer::ComponentBuffer,
        interpolate::Interpolate,
    },
    client_id::ClientId,
    component_sharing::{
        component_origin::ComponentOrigin,
        component_type_id::ComponentTypeId,
        receive_component_systems::ReceiveComponentSystems,
        send_component_systems::SendComponentSystems,
        share_component::{ComponentReadFilter, ShareComponent},
    },
    connection::{
        Connection,
        connection_id::ConnectionId,
        remote_action::{ComponentAction, RemoteAction},
    },
    entity_sharing::{
        receive_entity_systems::ReceiveEntitySystems,
        send_entity_systems::SendEntitySystems,
        sync_entity::{EntityReadFilter, EntityWriteFilter, SyncEntity},
    },
    event_sharing::session_events::{
        SessionAddedComponent, SessionRemovedComponent, SessionUpdatedComponent,
    },
};

pub struct BufferPlugin<TComponent, const COMPONENT_ID: u64, const BUFFER_SIIZE: usize>(
    PhantomData<TComponent>,
);

impl<TComponent, const COMPONENT_ID: u64, const BUFFER_SIIZE: usize> Default
    for BufferPlugin<TComponent, COMPONENT_ID, BUFFER_SIIZE>
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<
    TComponent: Component<Mutability = Mutable> + Serialize + DeserializeOwned + Clone + Interpolate,
    const COMPONENT_ID: u64,
    const BUFFER_SIIZE: usize,
> Plugin for BufferPlugin<TComponent, COMPONENT_ID, BUFFER_SIIZE>
{
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                Self::add_buffer_object
                    .in_set(SendBufferSystems::<TComponent>::default())
                    .before(Self::update_buffer_objects),
                Self::update_buffer_objects.in_set(SendBufferSystems::<TComponent>::default()),
                Self::update_buffer.in_set(RecieveBufferSystems::<TComponent>::default()),
                Self::send_owned.in_set(SendComponentSystems::<TComponent>::default()),
                Self::check_session_channels
                    .in_set(ReceiveComponentSystems::<TComponent>::default()),
            ),
        )
        .add_observer(Self::send_removed_owned)
        .configure_sets(
            FixedUpdate,
            RecieveBufferSystems::<TComponent>::default()
                .after(ReceiveComponentSystems::<TComponent>::default()),
        )
        .configure_sets(
            FixedUpdate,
            SendBufferSystems::<TComponent>::default()
                .before(SendComponentSystems::<TComponent>::default()),
        )
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
    TComponent: Component<Mutability = Mutable> + Serialize + DeserializeOwned + Clone + Interpolate,
    const COMPONENT_ID: u64,
    const BUFFER_SIZE: usize,
> BufferPlugin<TComponent, COMPONENT_ID, BUFFER_SIZE>
{
    fn add_buffer_object(
        no_buffers: Query<
            (Entity, &TComponent),
            (
                With<ShareComponent<TComponent>>,
                Without<BufferObject<TComponent>>,
                Without<ComponentOrigin<TComponent, ConnectionId>>,
            ),
        >,
        mut commands: Commands,
    ) {
        for (entity, component) in no_buffers {
            commands
                .entity(entity)
                .insert(BufferObject::<TComponent>::new(component.clone()));
        }
    }

    pub fn update_buffer_objects(
        buffer_objects: Query<(&mut BufferObject<TComponent>, &TComponent)>,
    ) {
        for (mut update_buffer, component) in buffer_objects {
            update_buffer.component = component.clone();
            update_buffer.index += 1;
        }
    }

    fn update_buffer(
        buffers: Query<
            (
                &mut ComponentBuffer<TComponent, BUFFER_SIZE>,
                &mut TComponent,
                Entity,
                &ComponentOrigin<TComponent, ConnectionId>,
            ),
            Allow<Disabled>,
        >,
        mut commands: Commands,
    ) {
        for (mut buffer, mut buffered_component, entity, origin) in buffers {
            buffer.try_pop();
            *buffered_component = buffer.actual.clone();
            commands.trigger(SessionUpdatedComponent::<TComponent> {
                entity,
                connection_id: origin.0,
                phantom: default(),
            });
        }
    }

    fn send_removed_owned(
        trigger: On<Remove, ShareComponent<TComponent>>,
        sync_entities: Query<
            (
                &SyncEntity,
                Option<&ComponentOrigin<TComponent, ClientId>>,
                Option<&EntityReadFilter<ConnectionId>>,
                &ComponentReadFilter<TComponent, ConnectionId>,
            ),
            (Allow<Disabled>, With<ShareComponent<TComponent>>),
        >,
        connections: Query<&mut Connection>,
        client_id: Res<ClientId>,
    ) {
        let Ok((removed_entity, origin_client, entity_filter, component_filter)) =
            sync_entities.get(trigger.entity)
        else {
            warn!("Removed Owner not found!");
            return;
        };

        let origin = origin_client.map_or(*client_id, |oc| oc.0);

        for mut session in connections {
            if let Some(entity_filter) = entity_filter
                && !entity_filter.is_allowed(&session.get_connection_id())
            {
                continue;
            }

            if !component_filter.is_allowed(&session.get_connection_id()) {
                continue;
            }

            session.component_removed(
                removed_entity.sync_id,
                ComponentTypeId(COMPONENT_ID),
                origin,
            );
        }
    }

    pub fn send_owned(
        owned_components: Query<
            (
                &mut ShareComponent<TComponent>,
                &BufferObject<TComponent>,
                &SyncEntity,
                Option<Ref<EntityReadFilter<ConnectionId>>>,
                Option<&ComponentOrigin<TComponent, ClientId>>,
                Ref<ComponentReadFilter<TComponent, ConnectionId>>,
            ),
            Allow<Disabled>,
        >,
        mut connections: Query<&mut Connection>,
        client_id: Res<ClientId>,
    ) {
        for (
            mut share_component,
            component,
            sync_entity,
            entity_filter,
            origin_client,
            connection_filter,
        ) in owned_components
        {
            let origin = origin_client.map_or(*client_id, |oc| oc.0);

            for mut connection in connections.iter_mut() {
                let is_component_allowed =
                    connection_filter.is_allowed(&connection.get_connection_id());
                let is_on = share_component
                    .on_sessions
                    .contains(&connection.get_connection_id());

                let is_entity_allowed =
                    entity_filter.map_or(true, |ef| ef.is_allowed(&connection.get_connection_id()));

                let is_allowed = is_component_allowed && is_entity_allowed;

                if is_allowed && !is_on {
                    connection.component_added(
                        sync_entity.sync_id,
                        ComponentTypeId(COMPONENT_ID),
                        component,
                        origin,
                    );
                    share_component
                        .on_sessions
                        .push(connection.get_connection_id());
                } else if is_allowed && is_on {
                    // info!("Sending update buffer: {:?}", component);
                    connection.component_updated(
                        sync_entity.sync_id,
                        ComponentTypeId(COMPONENT_ID),
                        component,
                        origin,
                    );
                } else if is_on && !is_allowed {
                    if is_entity_allowed {
                        connection.component_removed(
                            sync_entity.sync_id,
                            ComponentTypeId(COMPONENT_ID),
                            origin,
                        );
                    }
                    share_component.on_sessions = share_component
                        .on_sessions
                        .clone()
                        .into_iter()
                        .filter(|sid| *sid != connection.get_connection_id())
                        .collect();
                }
            }
        }
    }

    fn check_session_channels(
        connections: Query<&Connection>,
        mut sync_entites: Query<
            (
                &SyncEntity,
                Entity,
                Option<&EntityWriteFilter<ConnectionId>>,
                Option<&mut ComponentBuffer<TComponent, BUFFER_SIZE>>,
                Has<ShareComponent<TComponent>>,
            ),
            Allow<Disabled>,
        >,
        mut commands: Commands,
    ) {
        for connection in connections {
            let messages = connection.messages().filter(|ra| match ra {
                RemoteAction::Component {
                    action: _,
                    component_type_id,
                    entity_id: _,
                    client_id: _,
                } => component_type_id.0 == COMPONENT_ID,
                _ => false,
            });

            for message in messages {
                let RemoteAction::Component {
                    action,
                    component_type_id: _,
                    entity_id,
                    client_id,
                } = message
                else {
                    continue;
                };

                let Some((_, entity, entity_filter, data, should_share)) = sync_entites
                    .iter_mut()
                    .find(|(se, ..)| se.sync_id == *entity_id)
                else {
                    warn!("Entity for component event not found");
                    continue;
                };

                if !entity_filter.map_or(true, |ef| ef.is_allowed(&connection.get_connection_id()))
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
                            .remove::<ComponentBuffer<TComponent, BUFFER_SIZE>>()
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
                        if should_share {
                            for oc in connections.iter().filter(|oc| {
                                oc.get_connection_id() != connection.get_connection_id()
                            }) {
                                if let Err(error) = oc.messenger.send_action(message.clone()) {
                                    warn!(
                                        "Failed to relay action to connection. [{}] [{:?}] [{:?}]",
                                        error,
                                        oc.get_connection_id(),
                                        message
                                    );
                                }
                            }
                        }

                        let Some(buffer_object): Option<BufferObject<TComponent>> =
                            connection.get_data(component_data_raw)
                        else {
                            continue;
                        };

                        let Some(mut buffer) = data else {
                            let mut buffer = [const { None }; BUFFER_SIZE];
                            buffer[0] = Some(buffer_object.component.clone());

                            commands.entity(entity).insert((
                                buffer_object.component.clone(),
                                ComponentBuffer::<TComponent, BUFFER_SIZE> {
                                    actual: buffer_object.component.clone(),
                                    buffer,
                                    current_network_index: buffer_object.index,
                                },
                                ComponentOrigin::<TComponent, _>::new(
                                    connection.get_connection_id(),
                                ),
                                ComponentOrigin::<TComponent, _>::new(*client_id),
                            ));
                            commands.trigger(SessionAddedComponent::<TComponent> {
                                entity,
                                connection_id: connection.get_connection_id(),
                                phantom: default(),
                            });
                            continue;
                        };

                        buffer.set_component(buffer_object.index, buffer_object.component);
                    }
                }
            }
        }
    }
}
