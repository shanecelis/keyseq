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

* For the sake of finding key chords in code, prefer one way of describing the
  keys, e.g., accept "ctrl-A"; do not accept "control-A" or "C-A" or "Ctrl+A".

# Install

``` sh
cargo add keyseq --features bevy; # OR --features winit
```

# Principal Macros

* The `pkey!` macro specifies a physical key chord, e.g., `pkey! { ctrl-A }`.
* The `pkeyseq!` macro specifies a physical key chord sequence, e.g., `pkeyseq! { ctrl-A alt-B C }`.
* The `lkey!` macro specifies a logical key chord, .e.g, `lkey! { ctrl-a }`.
* The `lkeyseq!` macro specifies a logical key chord sequence, e.g. `lkeyseq! { ctrl-a alt-b c }`.

# Concepts

* A physical key denotes a particular key on the keyboard. It emits a key code
  that does not change no matter what modifiers are held down. For instance
  there is a physical 'Q' key, often to the right of the tab key. There is no
  physical lower-case 'q' key.
* A logical key is specified by the key produces. If pressing the key produces
  a 'q' character, then it is logically a 'q' key.
  
# Usage

## Winit

With the "winit" feature the `keyseq::winit::pkey!` macro returns a
`(Modifiers, KeyCode)` tuple.

### Physical Keys

```
use keyseq::{Modifiers, winit::pkey};
use winit::keyboard::KeyCode;

assert_eq!(pkey! { A },          (Modifiers::NONE,    KeyCode::KeyA));
assert_eq!(pkey! { ctrl-A },     (Modifiers::CONTROL, KeyCode::KeyA));
assert_eq!(pkey! { alt-A },      (Modifiers::ALT,     KeyCode::KeyA));
assert_eq!(pkey! { shift-A },    (Modifiers::SHIFT,   KeyCode::KeyA));
assert_eq!(pkey! { super-A },    (Modifiers::SUPER,   KeyCode::KeyA));
assert_eq!(pkey! { ctrl-alt-; }, (Modifiers::ALT |
                                  Modifiers::CONTROL, KeyCode::Semicolon));
```

### Physical Key Sequences

```
# use keyseq::Modifiers;
# use winit::keyboard::KeyCode;
use keyseq::winit::pkeyseq;
assert_eq!(pkeyseq! { A ctrl-B }, [(Modifiers::NONE,    KeyCode::KeyA),
                                  (Modifiers::CONTROL, KeyCode::KeyB)]);
```

### Logical Keys

With the "winit" feature the `keyseq::winit::lkey!` macro returns a
`(Modifiers, Key)` tuple.

```
use keyseq::{Modifiers, winit::lkey};
use winit::keyboard::Key;

assert_eq!(lkey! { a },          (Modifiers::NONE,    Key::Character('a')));
assert_eq!(lkey! { ctrl-a },     (Modifiers::CONTROL, Key::Character('a')));
assert_eq!(lkey! { alt-a },      (Modifiers::ALT,     Key::Character('a')));
assert_eq!(lkey! { shift-a },    (Modifiers::SHIFT,   Key::Character('a')));
assert_eq!(lkey! { super-a },    (Modifiers::SUPER,   Key::Character('a')));
assert_eq!(lkey! { ctrl-alt-; }, (Modifiers::ALT |
                                  Modifiers::CONTROL, Key::Character(';')));
```

### Logical Key Sequences
```
# use keyseq::Modifiers;
# use winit::keyboard::Key;
use keyseq::winit::lkeyseq;
assert_eq!(lkeyseq! { a ctrl-b }, [(Modifiers::NONE,    Key::Character('a')),
                                   (Modifiers::CONTROL, Key::Character('b'))]);
```

### No lower case physical keys

The following code will fail to compile. It insists on a capital 'A' for
specifying the A key.

```compile_fail
# use keyseq::winit::pkey;
let (mods, key) = pkey! { a }; // error: Use uppercase key names for physical keys
```

### Strict modifier order

With the "strict-order" feature enabled by default, modifiers out of order will
produce compiler errors. Without the feature, it will emit warnings.

```compile_fail
# use keyseq::winit::pkey;
let _ = pkey! { alt-ctrl-A }; // error: Modifiers must occur in this order: control, alt, shift, super.
```

### Why not use `winit::keyboard::ModifiersState`?

Why return `keyseq::Modifiers` and not `winit`'s own `ModifiersState`? Both
`keyseq::Modifiers` and `winit::keyboard::ModifiersState` are generated using
the [bitflags](https://docs.rs/bitflags/latest/bitflags/) crate. Originally this
crate did return `winit`'s native modifiers struct because it desugared to nearly
the same thing:

```ignore
// keyseq::winit::pkey! { ctrl-alt-A } desugared to
( ModifiersState::CONTROL 
| ModifiersState::ALT 
| ModifiersState::empty(), winit::keyboard::KeyCode::KeyA)

// keyseq::bevy::pkey! { ctrl-alt-A } desugars to
( Modifiers::CONTROL 
| Modifiers::ALT 
| Modifiers::empty(),      bevy::prelude::KeyCode::KeyA)
```

However, this these bitflags put together with bit-or pipes had a problem with
match expressions.

```ignore
let modifiers: ModifiersState = ...;
match (modifiers.into(), key_code) {
    // pkey! { ctrl-alt-A }    => println!("Just pressed ctrl-alt-A!"),
    // desugared to
    (ModifiersState::CONTROL | 
     ModifiersState::ALT | 
     ModifiersState::empty(), 
     KeyCode::KeyA)            => println!("Just pressed ctrl-alt-A!"),
```

When desugared the bit-or `|` is now interpretered as a match-or `|`, which does
not match `ctrl-alt`; it only matches `ctrl` or `alt` or no modifiers. (This
actually seems like a pretty big expressive deficiency for `bitflags` generated
structs.)

To avoid this problem `keyseq::Modifiers` is defined as `Modifiers(pub u8)` and
the bitflags are computed in the macro. That allows the following match
expressions to work as expected.

```ignore
match (modifiers.into(), key_code) {
    // pkey! { ctrl-alt-A }              => println!("Just pressed ctrl-alt-A!"),
    // now desugars to
    (Modifiers(3), KeyCode::KeyA)        => println!("Just pressed ctrl-alt-A!"),

    // And we can use the match-or to match multiple keychords.
    pkey! { ctrl-A } | pkey! { super-A } => println!("Just pressed ctrl-A or super-A!"),
```

In addition `keyseq::Modifiers` implements `From<ModifiersState>` and vice
versa.

## Bevy

With the "bevy" feature the `keyseq::bevy::pkey!` macro returns a
`(keyseq::Modifiers, KeyCode)` tuple.

Bevy doesn't have a logical key representation so there are no `lkey!` and
`lkeyseq!` macros.

```
use bevy::prelude::KeyCode;
use keyseq::{Modifiers, bevy::pkey};
assert_eq!(pkey! { ctrl-A },    (Modifiers::CONTROL, KeyCode::KeyA));
assert_eq!(pkey! { alt-A },     (Modifiers::ALT,     KeyCode::KeyA));
assert_eq!(pkey! { shift-A },   (Modifiers::SHIFT,   KeyCode::KeyA));
assert_eq!(pkey! { super-A },   (Modifiers::SUPER,   KeyCode::KeyA));
assert_eq!(pkey! { ctrl-shift-A }, 
                                (Modifiers::SHIFT |
                                 Modifiers::CONTROL, KeyCode::KeyA));
```

# Features

* winit, include support for winit
* bevy, include support for bevy
* poor, an anemic representation for internal testing
* strict-order, use a strict order for modifiers: ctrl, alt, shift, super
  (enabled by default)

# Examples

For both examples press `A` with modifiers and it will print a message showing
what keychord matched.

## Winit Example

``` sh
cargo run --example winit --features winit
```

## Bevy Example

``` sh
cargo run --example bevy --features bevy
```

# Notes

## Macro Notation 

Although using parens will work `pkey!(ctrl-alt-A)`, rustfmt will add spaces
around the hyphen changing it to `pkey!(ctrl - alt - A)`. Therefore, it's
suggested to use curly braces `pkey! { ctrl-alt-A }` which are not reformatted
like that.

## Compatibility

| keyseq | bevy   | winit  |
| ------ | ------ | ------ |
| 0.1.0  | 0.12.* | 0.29.* |
| 0.2.0  | 0.13.* | 0.29.* |

# License

This crate is licensed under the MIT License or the Apache License 2.0.

