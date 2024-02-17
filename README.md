# keyseq
![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![CI](https://github.com/shanecelis/keyseq/actions/workflows/rust.yml/badge.svg)](https://github.com/shanecelis/keyseq/actions)
  [![crates-io](https://img.shields.io/crates/v/keyseq.svg)](https://crates.io/crates/keyseq)
  [![api-docs](https://docs.rs/keyseq/badge.svg)](https://docs.rs/keyseq)

Specify key chords using `ctrl-A` short-hand, supports [bevy](https://bevyengine.org) and
[winit](https://github.com/rust-windowing/winit).

# Objective

* Specify key chords in code the same way as they are specified in
  documentation.

* For the sake of finding key chords in code, prefer one way of
  describing the keys, e.g., accept "ctrl-A"; do not accept "control-A" or "C-A"
  or "Ctrl+A".

# Install

``` sh
cargo add keyseq --features bevy
```

OR 

``` sh
cargo add keyseq --features winit
```

# Concepts

* Logical keys are specified by what the key does. If pressing the key produces
  a 'q', then it is logically a 'q' key.
* Physical keys are specified by the scan code the keyboard emits; they do
  not change. There is a physical 'Q' key, often to the right of the tab key.
  There is no physical lower-case 'q' key.
  
# Principal Macros

* The `key!` macro specifies a logical key chord.
* The `keyseq!` macro specifies a logical key chord sequence.
* The `pkey!` macro specifies a physical key chord.
* The `pkeyseq!` macro specifies a physical key chord sequence.

# Features

* winit, include support for winit macros
* bevy, include support for bevy macros
* poor, an anemic representation for internal testing
* strict-order, use a strict order for modifiers: ctrl, alt, shift, super (enabled by default)

## Winit

With the "winit" feature the `keyseq::winit::key!` macro returns a
`(ModifiersState, Key)` tuple.

### Logical Keys

```
use keyseq::{Modifiers, winit::key};
use winit::keyboard::Key;

assert_eq!(key!{ a },          (Modifiers::NONE,    Key::Character('a')));
assert_eq!(key!{ ctrl-a },     (Modifiers::CONTROL, Key::Character('a')));
assert_eq!(key!{ alt-a },      (Modifiers::ALT,     Key::Character('a')));
assert_eq!(key!{ shift-a },    (Modifiers::SHIFT,   Key::Character('a')));
assert_eq!(key!{ super-a },    (Modifiers::SUPER,   Key::Character('a')));
assert_eq!(key!{ ctrl-alt-; }, (Modifiers::ALT |
                                Modifiers::CONTROL, Key::Character(';')));
```

### Logical Key Sequences
```
# use keyseq::Modifiers;
# use winit::keyboard::Key;
use keyseq::winit::keyseq;
assert_eq!(keyseq!{ a ctrl-b}, [(Modifiers::NONE,    Key::Character('a')),
                                (Modifiers::CONTROL, Key::Character('b'))]);
```

### Physical Keys

```
# use keyseq::{Modifiers, winit::pkey};
use winit::keyboard::KeyCode;

assert_eq!(pkey!{ A },         (Modifiers::NONE,     KeyCode::KeyA));
```

The following code will fail to compile. It insists on a capital 'A' for
specifying the A key.

```compile_fail
# use keyseq::winit::pkey;
let (mods, key) = pkey!{ a }; // error: Use uppercase key names for physical keys
```

### Strict modifier order

With the "strict-order" feature, modifiers out of order will produce compiler
errors. Without the feature, it will emit warnings.

```compile_fail
# use keyseq::winit::pkey;
let _ = pkey!{ alt-ctrl-A }; // error: Modifiers must occur in this order: control, alt, shift, super.
```

## Bevy

With the "bevy" feature the `keyseq::bevy::pkey!` macro returns a
`(keyseq::Modifiers, KeyCode)` tuple.

Note: Bevy doesn't have a modifiers bit flag like Winit does. And Bevy doesn't
have a logical key representation yet (but there is one coming).

```
use bevy::prelude::KeyCode;
use keyseq::{Modifiers, bevy::pkey};
assert_eq!(pkey!{ ctrl-A },    (Modifiers::CONTROL, KeyCode::A));
assert_eq!(pkey!{ alt-A },     (Modifiers::ALT,     KeyCode::A));
assert_eq!(pkey!{ shift-A },   (Modifiers::SHIFT,   KeyCode::A));
assert_eq!(pkey!{ super-A },   (Modifiers::SUPER,   KeyCode::A));
assert_eq!(pkey!{ ctrl-shift-A }, 
                               (Modifiers::SHIFT |
                                Modifiers::CONTROL, KeyCode::A));
```

## Poor

The `keyseq::poor::key!` macro returns a `(u8, &str)` tuple to describe a key chord.

```
use keyseq::poor::key;
assert_eq!(key!{ A },       (0, "A"));
assert_eq!(key!{ ctrl-A },  (1, "A"));
assert_eq!(key!{ alt-A },   (2, "A"));
assert_eq!(key!{ shift-A }, (4, "A"));
assert_eq!(key!{ super-A }, (8, "A"));
```

The `keyseq::poor::keyseq!` macro returns a `[(u8, &str)]` array to describe a key
chord sequence.

```
use keyseq::poor::keyseq;
assert_eq!(keyseq!{ A B },             [(0, "A"), (0, "B")]);
assert_eq!(keyseq!{ shift-A shift-B }, [(4, "A"), (4, "B")]);
```

These particular representations are impractical since one would need to
interrogate untyped bitflags and string. The real use case requires features.

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

