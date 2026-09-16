use std::marker::PhantomData;

use bevy::{ecs::component::Mutable, prelude::*};
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    buffer::{
        buffer_object::BufferObject, buffer_systems::BufferSystems, buffered::Buffered,
        component_buffer::ComponentBuffer, interpolate::Interpolate,
    },
    component_sharing::{
        component_origin::ComponentOrigin, hookup_component_plugin::HookupComponentPlugin,
        receive_component_systems::ReceiveComponentSystems,
        send_component_systems::SendComponentSystems,
    },
    connection::connection_id::ConnectionId,
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
    TComponent: Component<Mutability = Mutable>
        + Serialize
        + DeserializeOwned
        + Clone
        + PartialEq
        + Interpolate,
    const COMPONENT_ID: u64,
    const BUFFER_SIIZE: usize,
> Plugin for BufferPlugin<TComponent, COMPONENT_ID, BUFFER_SIIZE>
{
    fn build(&self, app: &mut App) {
        app.add_plugins(HookupComponentPlugin::<
            BufferObject<TComponent>,
            COMPONENT_ID,
        >::default())
            .add_systems(
                FixedUpdate,
                (
                    Self::add_buffer_object.before(SendComponentSystems::<TComponent>::default()),
                    Self::update_buffer_objects
                        .before(SendComponentSystems::<TComponent>::default()),
                    Self::update_buffer.in_set(BufferSystems::<TComponent>::default()),
                    Self::add_buffer
                        .before(Self::update_buffer)
                        .in_set(BufferSystems::<TComponent>::default()),
                ),
            )
            .configure_sets(
                FixedUpdate,
                BufferSystems::<TComponent>::default()
                    .after(ReceiveComponentSystems::<TComponent>::default()),
            );
    }
}

impl<
    TComponent: Component<Mutability = Mutable>
        + Serialize
        + DeserializeOwned
        + Clone
        + PartialEq
        + Interpolate,
    const COMPONENT_ID: u64,
    const BUFFER_SIIZE: usize,
> BufferPlugin<TComponent, COMPONENT_ID, BUFFER_SIIZE>
{
    fn add_buffer_object(
        no_buffers: Query<(Entity, &TComponent), Without<BufferObject<TComponent>>>,
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
            if update_buffer.component == *component && !update_buffer.last_changed {
                continue;
            }

            update_buffer.last_changed = update_buffer.component != *component;
            update_buffer.component = component.clone();
            update_buffer.index += 1;
        }
    }

    fn add_buffer(
        missing_buffer: Query<
            (Entity, &BufferObject<TComponent>),
            (
                Without<ComponentBuffer<TComponent, BUFFER_SIIZE>>,
                With<ComponentOrigin<TComponent, ConnectionId>>,
            ),
        >,
        mut commands: Commands,
    ) {
        for (entity, buffer_object) in missing_buffer {
            let mut buffer = [const { None }; BUFFER_SIIZE];
            buffer[0] = Some(buffer_object.component.clone());

            commands.entity(entity).insert((
                ComponentBuffer::<TComponent, BUFFER_SIIZE> {
                    actual: buffer_object.component.clone(),
                    buffer,
                    current_network_index: buffer_object.index,
                },
                Buffered::<TComponent>(buffer_object.component.clone()),
            ));
        }
    }

    fn update_buffer(
        buffers: Query<(
            &mut ComponentBuffer<TComponent, BUFFER_SIIZE>,
            &mut Buffered<TComponent>,
            Ref<BufferObject<TComponent>>,
        )>,
    ) {
        for (mut buffer, mut buffered, buffer_object) in buffers {
            if buffer_object.is_changed() {
                buffer.set_component(buffer_object.index, buffer_object.component.clone());
            }

            buffer.try_pop();
            buffered.0 = buffer.actual.clone();
        }
    }
}
