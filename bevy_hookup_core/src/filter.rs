use bevy::reflect::Reflect;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Debug, Reflect, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Filter<T> {
    Blacklist(Vec<T>),
    Whitelist(Vec<T>),
}

impl<T: PartialEq> Filter<T> {
    pub fn allow_all() -> Self {
        Self::Blacklist(vec![])
    }

    pub fn allow_none() -> Self {
        Self::Whitelist(vec![])
    }

    pub fn is_allowed(&self, entry: &T) -> bool {
        match self {
            Filter::Blacklist(items) => !items.contains(entry),
            Filter::Whitelist(items) => items.contains(entry),
        }
    }

    pub fn merge(self, other: Self) -> Self {
        match self {
            Filter::Blacklist(self_items) => match other {
                Filter::Blacklist(other_items) => Filter::Blacklist(
                    self_items
                        .into_iter()
                        .chain(other_items.into_iter())
                        .dedup()
                        .collect_vec(),
                ),
                Filter::Whitelist(other_items) => Filter::Whitelist(
                    other_items
                        .into_iter()
                        .filter(|item| !self_items.contains(item))
                        .collect_vec(),
                ),
            },
            Filter::Whitelist(self_items) => match other {
                Filter::Blacklist(other_items) => Filter::Whitelist(
                    self_items
                        .into_iter()
                        .filter(|item| !other_items.contains(item))
                        .collect_vec(),
                ),
                Filter::Whitelist(other_items) => Filter::Whitelist(
                    self_items
                        .into_iter()
                        .filter(|item| other_items.contains(item))
                        .collect_vec(),
                ),
            },
        }
    }
}
