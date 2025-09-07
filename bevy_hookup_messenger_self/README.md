# Self Messenger

The simpelst implementation of a messenger, this just streams the shared components into the same world. To make sure they end up on different entities, the Entity IDs are changed before being sent to the core library again.

This implementation can be used for singleplayer modes.

## Example

All you have to do is spawn a self session into your world.

You need an implementation of `TSendables` like with any other session implementation.

```rust
commands.spawn(SelfSession::<Sendables>::new().to_session());
```