#![doc(html_root_url = "https://docs.rs/keyseq/0.2.2")]
// The README is written with code that requires both winit and bevy features.
#![cfg_attr(all(feature = "winit", feature = "bevy"),
            doc = include_str!("../README.md"))]
#![cfg_attr(
    not(all(feature = "winit", feature = "bevy")),
    doc = "Warning: Not full documentation. Please generate doc with `--all-features` option to include README."
)]
use bitflags::bitflags;
use std::fmt;

/// keyseq macros that are a "poor" representation but useful for internal tests.
#[rustfmt::skip]
#[cfg(feature = "poor")]
pub mod poor {
//! ## Poor
//!
//! With the "poor" feature the `keyseq::poor::key!` macro returns a `(u8, &str)`
//! tuple to describe a key chord.
//!
//! ```
//! use keyseq::poor::lkey;
//! assert_eq!(lkey! { A },       (0, "A"));
//! assert_eq!(lkey! { ctrl-A },  (1, "A"));
//! assert_eq!(lkey! { alt-A },   (2, "A"));
//! assert_eq!(lkey! { shift-A }, (4, "A"));
//! assert_eq!(lkey! { super-A }, (8, "A"));
//! ```
//!
//! The `keyseq::poor::lkeyseq!` macro returns a `[(u8, &str)]` array to describe a key
//! chord sequence.
//!
//! ```
//! use keyseq::poor::lkeyseq;
//! assert_eq!(lkeyseq! { A B },             [(0, "A"), (0, "B")]);
//! assert_eq!(lkeyseq! { shift-A shift-B }, [(4, "A"), (4, "B")]);
//! ```
//!
//! These particular representations are impractical since one would need to
//! interrogate untyped bitflags and string. The real use case requires features.
    pub use keyseq_macros::{poor_lkey as lkey,
                            poor_lkeyseq as lkeyseq,
                            poor_pkey as pkey,
                            poor_pkeyseq as pkeyseq};
}

/// A bit flag that stores the modifier keys--control, alt, shift, and
/// super--in a byte.
#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Hash, Ord)]
pub struct Modifiers(pub u8);

bitflags! {
    impl Modifiers: u8 {
        /// Represents no modifier keys.
        const NONE    = 0b00000000;
        /// Represents the control key, left or right.
        const CONTROL = 0b00000001;
        /// Represents the alt key, left or right.
        const ALT     = 0b00000010;
        /// Represents the shift key, left or right.
        const SHIFT   = 0b00000100;
        /// Represents the macOS command or Windows key, left or right.
        const SUPER   = 0b00001000;
    }
}

impl fmt::Debug for Modifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // f.debug_tuple("Modifiers")
        //     .field(&self.0)
        //     .finish()

        // Fake the debug_tuple() function.
        f.write_str("Modifiers(")?;
        if self.is_empty() {
            f.write_str("NONE")?;
        } else {
            fmt::Display::fmt(self, f)?;
        }
        f.write_str(")")
    }
}

impl fmt::Display for Modifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut accum = Vec::new();
        if self.contains(Modifiers::CONTROL) {
            accum.push("ctrl");
        }
        if self.contains(Modifiers::ALT) {
            accum.push("alt");
        }
        if self.contains(Modifiers::SHIFT) {
            accum.push("shift");
        }
        if self.contains(Modifiers::SUPER) {
            accum.push("super");
        }
        f.write_str(&accum.join("-"))
    }
}

impl From<u8> for Modifiers {
    fn from(x: u8) -> Modifiers {
        Modifiers::from_bits(x).expect("Should only have first nybble set")
    }
}

#[cfg(feature = "winit")]
pub mod winit;

#[cfg(feature = "bevy")]
pub mod bevy;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn display_modifiers() {
        let mods = Modifiers(1 + 2 + 4);
        assert_eq!(format!("{}", mods), "ctrl-alt-shift");

    }
}
