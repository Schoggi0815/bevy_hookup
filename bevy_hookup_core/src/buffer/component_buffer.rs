use bevy::prelude::*;

use crate::buffer::interpolate::Interpolate;

#[derive(Component, Debug, Reflect)]
pub struct ComponentBuffer<TComponent, const BUFFER_SIZE: usize> {
    pub buffer: [Option<TComponent>; BUFFER_SIZE],
    pub current_network_index: u64,
    pub actual: TComponent,
}

impl<TComponent: Clone + Interpolate, const BUFFER_SIZE: usize>
    ComponentBuffer<TComponent, BUFFER_SIZE>
{
    pub fn try_pop(&mut self) {
        if let Some(next) = self.buffer[0].take() {
            self.actual = next;
        } else {
            let Some((latest, latest_index)) = self.get_first_future() else {
                return;
            };

            let distance = latest_index + 1;
            let t = 1.0 / distance as f32;

            self.actual = self.actual.interpolate(&latest, t);
        }

        self.current_network_index += 1;
        self.shift_left();
    }

    pub fn set_component(&mut self, network_index: u64, component: TComponent) {
        if network_index <= self.current_network_index {
            return;
        }

        let array_index = (network_index - self.current_network_index - 1) as usize;

        if array_index < BUFFER_SIZE {
            self.buffer[array_index] = Some(component);
            return;
        }

        self.resync(network_index, component);
    }

    fn get_first_future(&self) -> Option<(TComponent, usize)> {
        self.buffer
            .iter()
            .enumerate()
            .find_map(|(index, component)| component.clone().map(|component| (component, index)))
    }

    fn shift_left(&mut self) {
        for i in 0..BUFFER_SIZE.saturating_sub(1) {
            self.buffer[i] = self.buffer[i + 1].take();
        }

        if BUFFER_SIZE > 0 {
            self.buffer[BUFFER_SIZE - 1] = None;
        }
    }

    fn resync(&mut self, network_index: u64, component: TComponent) {
        self.buffer = std::array::from_fn(|_| None);

        self.actual = component;
        self.current_network_index = network_index;
    }
}
