# BEVY HOOKUP

[![Following released Bevy versions](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://bevy.org/learn/quick-start/plugin-development/#main-branch-tracking)
[![crates.io](https://img.shields.io/crates/v/bevy_hookup_core)](https://crates.io/crates/bevy_hookup_core)
[![docs.rs](https://docs.rs/bevy_hookup_core/badge.svg)](https://docs.rs/bevy_hookup_core)

A simple library for syncing bevy components over arbitrary sessions.

This repo consists of multiple crates:
- `bevy_hookup_core`: The core part of the syncing logic, not very useful without a messenger implementation.
- `bevy_hookup_messenger_websocket`: A websocket implementation for the session. It has a server and client part, but they are only different when building the connection, the sharing of entities and components works in both directions the same way.
- `bevy_hookup_messenger_steamworks`: A steamworks implementation for the session. This uses bevy-steamworks as a base to implement peer to peer messenging.

The core library supports practically any network topology and protocol and leaves the decision of the networking architecture on the user.

| bevy  | bevy_hookup_core | bevy_hookup_messenger_websocker | bevy_hookup_messenger_steamworks |
|-------|------------------|---------------------------------|----------------------------------|
| 0.19  | 6.0.0            | 3.0.0                           | 2.0.0                            |

## Usage

For usage of the basic core library see the [bevy hookup core Readme](bevy_hookup_core/README.md).

You will also need to use any messenger or implement your own.
Check out the usages on the existing ones here:

[bevy hookup messenger websocket](bevy_hookup_messenger_websocket/README.md)

[bevy hookup messenger steamworks](bevy_hookup_messenger_steamworks/README.md)

## Examples

There is also a simple example that you can reference [here](examples/basic/src/main.rs).

The example uses the websocket messenger for simplicity, but you can of course just swap that out with any messenger you like.