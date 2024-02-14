use bitflags::bitflags;

pub use keyseq_macros::{key,
                        keyseq,
                        pkey,
                        pkeyseq};

bitflags! {
    /// A bit flag that stores the modifier keys--alt, control, shift, and
    /// super--in a byte.
    #[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Eq, Hash, Ord)]
    pub struct Modifiers: u8 {
        /// Represents the shift key, left or right.
        const SHIFT   = 0b00000001;
        /// Represents the control key, left or right.
        const CONTROL = 0b00000010;
        /// Represents the alt key, left or right.
        const ALT     = 0b00000100;
        /// Represents the macOS command or Windows key, left or right.
        const SUPER   = 0b00001000;
    }
}

impl From<u8> for Modifiers {
    fn from(x: u8) -> Modifiers {
        Modifiers::from_bits(x).unwrap()
    }
}

#[cfg(feature = "bevy")]
pub mod bevy;

#[cfg(feature = "winit")]
pub mod winit {
    pub use keyseq_macros::{winit_key as key,
                            winit_keyseq as keyseq,
                            winit_pkey as pkey,
                            winit_pkeyseq as pkeyseq};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(key!(A), (0, "A"));
    }

}
