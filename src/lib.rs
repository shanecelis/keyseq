#![doc(html_root_url = "https://docs.rs/keyseq/0.7.0")]
// The README is written with code that requires both winit and bevy features.
#![cfg_attr(all(feature = "winit", feature = "bevy"),
            doc = include_str!("../README.md"))]
#![cfg_attr(
    not(all(feature = "winit", feature = "bevy")),
    doc = "Warning: Not full documentation. Please generate doc with `--all-features` option to include README."
)]
#![forbid(missing_docs)]
use bitflags::bitflags;
use std::fmt;

#[rustfmt::skip]
#[cfg(feature = "poor")]
pub mod poor {
//! keyseq macros that are a "poor" representation but useful for internal tests.
//!
//! ## Poor
//!
//! With the "poor" feature the `keyseq::poor::key!` macro returns a `(u8, &str)`
//! tuple to describe a key chord.
//!
//! ```
//! use keyseq::poor::lkey;
//! assert_eq!(lkey! { A },       (0, "A"));
//! assert_eq!(lkey! { Ctrl-A },  (1, "A"));
//! assert_eq!(lkey! { Alt-A },   (2, "A"));
//! assert_eq!(lkey! { Shift-A }, (4, "A"));
//! assert_eq!(lkey! { Super-A }, (8, "A"));
//! ```
//!
//! The `keyseq::poor::lkeyseq!` macro returns a `[(u8, &str)]` array to describe a key
//! chord sequence.
//!
//! ```
//! use keyseq::poor::lkeyseq;
//! assert_eq!(lkeyseq! { A B },             [(0, "A"), (0, "B")]);
//! assert_eq!(lkeyseq! { Shift-A Shift-B }, [(4, "A"), (4, "B")]);
//! ```
//!
//! These particular representations are impractical since one would need to
//! interrogate untyped bitflags and a string. The real use case requires
//! features.
    pub use keyseq_macros::{poor_lkey as lkey,
                            poor_lkeyseq as lkeyseq,
                            poor_pkey as pkey,
                            poor_pkeyseq as pkeyseq};
}

/// A bit flag that stores the modifier keys--control, Alt, Shift, and
/// Super--in a byte.
#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Hash, Ord)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct Modifiers(pub u8);

bitflags! {
    impl Modifiers: u8 {
        /// Represents no modifier keys.
        const NONE    = 0b00000000;
        /// Represents the control key, left or right.
        const CONTROL = 0b00000001;
        /// Represents the Alt key, left or right.
        const ALT     = 0b00000010;
        /// Represents the Shift key, left or right.
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
        let mut first = true;
        if self.contains(Modifiers::CONTROL) {
            f.write_str("Ctrl")?;
            first = false;
        }
        if self.contains(Modifiers::ALT) {
            if !first {
                f.write_str("-")?;
            }
            f.write_str("Alt")?;
            first = false;
        }
        if self.contains(Modifiers::SHIFT) {
            if !first {
                f.write_str("-")?;
            }
            f.write_str("Shift")?;
            first = false;
        }
        if self.contains(Modifiers::SUPER) {
            if !first {
                f.write_str("-")?;
            }
            f.write_str("Super")?;
        }
        Ok(())
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

/// This exists merely to run this compile fail test.
///
/// ```compile_fail
/// assert_eq!(poor::pkey! { Ctrl+A }, (1, "A"));
/// ```
#[cfg(all(feature = "poor", not(feature = "permit-plus")))]
struct test_dummy;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn display_modifiers() {
        let mods = Modifiers(1 + 2 + 4);
        assert_eq!(format!("{}", mods), "Ctrl-Alt-Shift");
    }

    #[cfg(all(feature = "poor", feature = "permit-plus"))]
    #[test]
    fn permit_plus() {
        assert_eq!(poor::pkey! { Ctrl+A }, (1, "A"));
    }
}
