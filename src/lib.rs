#![doc(html_root_url = "https://docs.rs/keyseq/0.1.0")]
#![doc = include_str!("../README.md")]
use bitflags::bitflags;
use std::fmt;

/// keyseq macros that are a "poor" representation but useful for internal tests.
#[cfg(feature = "poor")]
pub mod poor {
    pub use keyseq_macros::{poor_key as key,
                            poor_keyseq as keyseq,
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
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
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
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        ::bitflags::parser::to_writer(self, f)
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

