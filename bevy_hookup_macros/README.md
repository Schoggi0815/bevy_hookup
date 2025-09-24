# Bevy Hookup Macros

This crate provides a simple helper macro for your `Sendables` enum, to implement the needed `From<&TComponent>` and `Into<Option<TComponent>>` traits.

## Example

```rust
#[derive(Clone, Sendable, Serialize, Deserialize)]
pub enum Sendables {
    #[sendable]
    SyncName(SyncName),
}
```

This will create the following implementations:

```rust
impl From<&SyncName> for Sendables {
    fn from(value: &SyncName) -> Self {
        Self::SyncName(value.clone())
    }
}

impl Into<Option<SyncName>> for Sendables {
    fn into(self) -> Option<SyncName> {
        match self {
            Self::SyncName(value) => Some(value),
            _ => None,
        }
    }
}
```