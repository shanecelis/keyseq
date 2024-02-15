# keyseq_macros
![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![CI](https://github.com/shanecelis/keyseq_macros/actions/workflows/rust.yml/badge.svg)](https://github.com/shanecelis/keyseq_macros/actions)
  [![crates-io](https://img.shields.io/crates/v/keyseq_macros.svg)](https://crates.io/crates/keyseq_macros)
  [![api-docs](https://docs.rs/keyseq_macros/badge.svg)](https://docs.rs/keyseq_macros)

Specify key chords using a short-hand, e.g., `ctrl-A`, for the [bevy game engine](https://bevyengine.org) or [winit](https://github.com/rust-windowing/winit).

# Install

``` sh
cargo add keyseq_macros
```

# Concepts

* Logical keys are specified by what the key does. If pressing the key produces
  a 'q', then it is logically a 'q' key.
* Physical keys are specified by the scan code the keyboard emits; they do
  not change. There is a physical 'Q' key, often to the right of the tab key.
  There is no physical lower-case 'q' key.
  
# Principal Macros

* The `key!` macro specifies a logical key chord.
* The `keyseq!` macro specifies logical key chord sequences.
* The `pkey!` macro specifies a physical key chord.
* The `pkeyseq!` macro specifies physical key chord sequences.

# Usage

The `key!` macro returns a `(u8, &str)` tuple to describe a key chord. This is
impractical in most cases since one would need to interrogate the untyped
bitflags[^1] and string. The real use case comes with its features.

```
use keyseq_macros::key;
assert_eq!(key!(A), (0, "A"));
assert_eq!(key!(ctrl-A), (1, "A"));
assert_eq!(key!(alt-A), (2, "A"));
assert_eq!(key!(shift-A), (4, "A"));
assert_eq!(key!(super-A), (8, "A"));
```

The `keyseq!` macro returns a `[(u8, &str)]` array to describe a key chord sequence.

```
use keyseq_macros::keyseq;
assert_eq!(keyseq!(A B), [(0, "A"), (0, "B")]);
assert_eq!(keyseq!(shift-A shift-B), [(4, "A"), (4, "B")]);
```
# Features

* winit
* bevy

## Winit

With the "winit" feature the `winit_key!` macro returns a `(ModifiersState, Key)` tuple.

```
use keyseq_macros::winit_key as key;
use winit::keyboard::{ModifiersState, Key};
assert_eq!(key!(a), (ModifiersState::empty(), Key::Character('a')));
assert_eq!(key!(shift-a), (ModifiersState::SHIFT, Key::Character('a')));
assert_eq!(key!(ctrl-a), (ModifiersState::CONTROL, Key::Character('a')));
assert_eq!(key!(alt-a), (ModifiersState::ALT, Key::Character('a')));
assert_eq!(key!(super-a), (ModifiersState::SUPER, Key::Character('a')));
assert_eq!(key!(ctrl-alt-;), (ModifiersState::ALT | ModifiersState::CONTROL, Key::Character(';')));
```

## Bevy

With the "bevy" feature the `bevy_pkey_u8!` macro returns a `(u8, KeyCode)` tuple.

Note: Bevy doesn't have a modifiers bit flag like Winit does. And Bevy doesn't
have a logical key representation yet (but there is one coming).

```
use bevy::prelude::KeyCode;
use keyseq_macros::bevy_pkey_u8 as pkey;
assert_eq!(pkey!(ctrl-A), (1, KeyCode::A));
assert_eq!(pkey!(alt-A), (2, KeyCode::A));
assert_eq!(pkey!(shift-A), (4, KeyCode::A));
assert_eq!(pkey!(super-A), (8, KeyCode::A));
assert_eq!(pkey!(ctrl-shift-A), (5, KeyCode::A));
```

# Examples

## Winit Example
Run a winit-based example:

``` sh
cargo run --example winit
```

Press `1` or `shift-1` and it will print a message.

## Bevy Example

``` sh
cargo run --example bevy
```

This will show a rotating cube with the shader as its surfaces.

# License

This crate is licensed under the MIT License or the Apache License 2.0.

[^1]: proc_macro crates cannot specify anything but procedural macros. If they
    could, keyseq_macros would include a modifiers bitflag struct.
