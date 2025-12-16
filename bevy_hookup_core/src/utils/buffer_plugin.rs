use std::marker::PhantomData;

use bevy::{ecs::component::Mutable, prelude::*};

use crate::{
    from_session::FromSession,
    hookup_component_plugin::HookupComponentPlugin,
    receive_component_systems::ReceiveComponentSystems,
    send_component_systems::SendComponentSystems,
    utils::{
        buffer_object::BufferObject, buffer_systems::BufferSystems, buffered::Buffered,
        component_buffer::ComponentBuffer, interpolate::Interpolate,
    },
};

pub struct BufferPlugin<TComponent, TSendables, const BUFFER_SIIZE: usize>(
    PhantomData<TComponent>,
    PhantomData<TSendables>,
);

impl<TComponent, TSendables, const BUFFER_SIIZE: usize> Default
    for BufferPlugin<TComponent, TSendables, BUFFER_SIIZE>
{
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}

impl<
    TSendables: Send
        + Sync
        + 'static
        + Clone
        + for<'a> From<&'a BufferObject<TComponent>>
        + Into<Option<BufferObject<TComponent>>>,
    TComponent: Sync + Send + Component<Mutability = Mutable> + Clone + Interpolate + 'static,
    const BUFFER_SIIZE: usize,
> Plugin for BufferPlugin<TComponent, TSendables, BUFFER_SIIZE>
{
    fn build(&self, app: &mut App) {
        app.add_plugins(HookupComponentPlugin::<TSendables, BufferObject<TComponent>>::default())
            .add_systems(
                FixedUpdate,
                (
                    Self::add_buffer_object.before(SendComponentSystems::<TComponent>::default()),
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

impl<TComponent: Component + Clone + Interpolate, TSendables, const BUFFER_SIIZE: usize>
    BufferPlugin<TComponent, TSendables, BUFFER_SIIZE>
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

    fn add_buffer(
        missing_buffer: Query<
            (Entity, &BufferObject<TComponent>),
            (
                Without<ComponentBuffer<TComponent, BUFFER_SIIZE>>,
                With<FromSession>,
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
