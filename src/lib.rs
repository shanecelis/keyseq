use bitflags::bitflags;
#[cfg(feature = "bevy")]
use bevy::input::keyboard::{Input, KeyCode};
// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }
pub use keyseq_macros::{key,
                        keyseq,
                        pkey,
                        pkeyseq};

bitflags! {
    /// A bit flag that stores the modifier keys--alt, control, shift, and super--in a byte.
    #[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Eq, Hash, Ord)]
    pub struct Modifiers: u8 {
        /// Represents the alt key, left or right.
        const ALT     = 0b00000001;
        /// Represents the control key, left or right.
        const CONTROL = 0b00000010;
        /// Represents the shift key, left or right.
        const SHIFT   = 0b00000100;
        /// Represents the macOS command or Windows key, left or right.
        const SUPER   = 0b00001000;
    }
}

#[cfg(feature = "bevy")]
impl Modifiers {
    /// Check modifier keys for `any_pressed()` to populate bit flags.
    pub fn from_input(input: &Input<KeyCode>) -> Modifiers {
        let mut mods = Modifiers::empty();
        if input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
            mods |= Modifiers::SHIFT;
        }
        if input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
            mods |= Modifiers::CONTROL;
        }
        if input.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]) {
            mods |= Modifiers::ALT;
        }
        if input.any_pressed([KeyCode::SuperLeft, KeyCode::SuperRight]) {
            mods |= Modifiers::SUPER;
        }
        mods
    }
}

impl From<KeyCode> for Modifiers {
    #[inline(always)]
    fn from(key: KeyCode) -> Self {
        match key {
            KeyCode::ShiftLeft | KeyCode::ShiftRight => Modifiers::SHIFT,
            KeyCode::ControlLeft | KeyCode::ControlRight => Modifiers::CONTROL,
            KeyCode::AltLeft | KeyCode::AltRight => Modifiers::ALT,
            KeyCode::SuperLeft | KeyCode::SuperRight => Modifiers::SUPER,
            _ => Modifiers::empty(),
        }
    }
}

#[cfg(feature = "bevy")]
pub mod bevy {
    pub use keyseq_macros::{bevy_pkey as pkey,
                            bevy_pkeyseq as pkeyseq};
}

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
