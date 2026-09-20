use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use bevy::prelude::*;

use crate::connection::remote_action::{EventId, EventTimestamp};

#[derive(Debug, Resource, Reflect)]
pub struct EventMap {
    pub map: HashMap<EventId, EventTimestamp>,
    pub retention_time_secs: u64,
}

impl Default for EventMap {
    fn default() -> Self {
        Self {
            map: Default::default(),
            retention_time_secs: 120,
        }
    }
}

impl EventMap {
    fn cutoff_timestamp(&self) -> EventTimestamp {
        EventTimestamp(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                - self.retention_time_secs,
        )
    }

    pub fn valid(&self, id: &EventId, timestamp: &EventTimestamp) -> bool {
        if timestamp < &self.cutoff_timestamp() {
            return false;
        };

        !self.map.contains_key(id)
    }

    pub fn insert(&mut self, id: EventId, timestamp: EventTimestamp) {
        self.map.insert(id, timestamp);
    }

    pub fn clear_old(&mut self) {
        let mut cutoff_timestamp = self.cutoff_timestamp();
        self.map
            .retain(|_, timestamp| timestamp >= &mut cutoff_timestamp);
    }
}
