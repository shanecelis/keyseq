# keyseq_macros
![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![CI](https://github.com/shanecelis/keyseq_macros/actions/workflows/rust.yml/badge.svg)](https://github.com/shanecelis/keyseq_macros/actions)
  [![crates-io](https://img.shields.io/crates/v/keyseq_macros.svg)](https://crates.io/crates/keyseq_macros)
  [![api-docs](https://docs.rs/keyseq_macros/badge.svg)](https://docs.rs/keyseq_macros)

Specify key chords using a short-hand, e.g., `ctrl-A`, for the [bevy game engine](https://bevyengine.org) or [winit](https://github.com/rust-windowing/winit).

# Description

This crate is the an internal crate of procedural macros for the `keyseq` crate. See that crate for documentation, tests, and examples.

# Principal Macros

* The `key!` macro specifies a logical key chord.
* The  lkeyseq!` macro specifies logical key chord sequences.
* The `pkey!` macro specifies a physical key chord.
* The `pkeyseq!` macro specifies physical key chord sequences.

# Usage

The `key!` macro returns a `(u8, &str)` tuple to describe a key chord. This is
impractical in most cases since one would need to interrogate the untyped
bitflags[^1] and string. The real use case comes with its features.

```
use keyseq_macros::poor_key as key;
assert_eq!(key!(A), (0, "A"));
assert_eq!(key!(ctrl-A), (1, "A"));
assert_eq!(key!(alt-A), (2, "A"));
assert_eq!(key!(shift-A), (4, "A"));
assert_eq!(key!(super-A), (8, "A"));
```

The `keyseq!` macro returns a `[(u8, &str)]` array to describe a key chord sequence.

```
use keyseq_macros::poor_keyseq as keyseq;
assert_eq!(keyseq!(A B), [(0, "A"), (0, "B")]);
assert_eq!(keyseq!(shift-A shift-B), [(4, "A"), (4, "B")]);
```
# Features

* winit
* bevy

## Winit

With the "winit" feature the `winit_key!` macro returns a `(ModifiersState, Key)` tuple.


## Bevy

With the "bevy" feature the `bevy_pkey!` macro returns a `(keyseq::Modifiers, KeyCode)` tuple.

Note: Bevy doesn't have a modifiers bit flag like Winit does. And Bevy doesn't
have a logical key representation yet (but there is one coming).

# License

This crate is licensed under the MIT License or the Apache License 2.0.

[^1]: proc_macro crates cannot specify anything but procedural macros. If they
    could, keyseq_macros would include a modifiers bitflag struct.
