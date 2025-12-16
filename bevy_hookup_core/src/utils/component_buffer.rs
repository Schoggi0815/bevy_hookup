use bevy::prelude::*;
use itertools::{FoldWhile, Itertools};

use crate::utils::interpolate::Interpolate;

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
        let Some(ref first) = self.buffer[0] else {
            return;
        };

        self.actual = first.clone();

        for i in 0..BUFFER_SIZE - 1 {
            self.buffer[i] = self.buffer[i + 1].clone();
        }

        self.buffer[BUFFER_SIZE - 1] = None;
        self.current_network_index += 1;
    }

    pub fn set_component(&mut self, network_index: u64, component: TComponent) {
        let array_index = (network_index - self.current_network_index) as usize;

        if array_index < BUFFER_SIZE {
            self.buffer[array_index] = Some(component.clone());

            if array_index > 0 && self.buffer[array_index - 1].is_none() {
                let (latest_position, latest_index) = self.get_latest();

                for i in (latest_index + 1) as usize..array_index {
                    let lerp_value = (i as f32 - latest_index as f32)
                        / (array_index as f32 - latest_index as f32);

                    self.buffer[i] = Some(latest_position.interpolate(&component, lerp_value));
                }
            }

            return;
        }

        let shift = array_index - BUFFER_SIZE + 1;
        let array_index = BUFFER_SIZE - 1;

        let (latest_position, latest_index) = self.get_latest();
        let latest_index = latest_index - shift as i32;

        let lerp_max = array_index as f32 - latest_index as f32;

        let actual_current_index = shift - 1;
        let actual_current = self.buffer.get(actual_current_index).cloned().flatten();

        self.actual = if let Some(actual_current) = actual_current {
            actual_current
        } else {
            latest_position.interpolate(
                &component,
                (actual_current_index as f32 - latest_index as f32) / lerp_max,
            )
        };

        for i in 0..BUFFER_SIZE - 1 {
            let before_index = i + shift;
            let new = self.buffer.get(before_index).cloned().flatten();

            self.buffer[i] = if let Some(new) = new {
                Some(new)
            } else {
                Some(latest_position.interpolate(
                    &component,
                    (before_index as f32 - latest_index as f32) / lerp_max,
                ))
            };
        }

        self.buffer[array_index] = Some(component);
        self.current_network_index += shift as u64;
    }

    fn get_latest(&self) -> (TComponent, i32) {
        self.buffer
            .iter()
            .enumerate()
            .fold_while((self.actual.clone(), -1), |acc, (index, current)| {
                if let Some(current) = current {
                    FoldWhile::Continue((current.clone(), index as i32))
                } else {
                    FoldWhile::Done(acc)
                }
            })
            .into_inner()
    }
}
