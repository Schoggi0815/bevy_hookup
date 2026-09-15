use bevy::reflect::Reflect;
use itertools::Itertools;

#[derive(Debug, Reflect, Clone, PartialEq, Eq)]
pub enum Filter<T> {
    AllowAll,
    AllowNone,
    Blacklist(Vec<T>),
    Whitelist(Vec<T>),
}

impl<T: PartialEq> Filter<T> {
    pub fn is_allowed(&self, entry: &T) -> bool {
        match self {
            Filter::AllowAll => true,
            Filter::AllowNone => false,
            Filter::Blacklist(items) => !items.contains(entry),
            Filter::Whitelist(items) => items.contains(entry),
        }
    }

    pub fn merge(self, other: Self) -> Self {
        if matches!(other, Self::AllowAll) {
            return self;
        }

        match self {
            Filter::AllowAll => other,
            Filter::AllowNone => self,
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
                _ => other,
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
                _ => other,
            },
        }
    }
}
