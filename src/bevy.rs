use super::Modifiers;
use ::bevy::input::{Input, keyboard::{KeyCode}};

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

pub use keyseq_macros::{bevy_pkey as pkey,
                        bevy_pkeyseq as pkeyseq};

// #[macro_export]
// macro_rules! bevy_mod_pkeyseq {
//     ($($key:tt)*) => {
//         { let array = ::keyseq_macros::bevy_pkeyseq!($($key)*);
//           array.into_iter().map(|(mods, key)| (Modifiers::from_bits(mods).unwrap(), key))

//             (Modifiers::from_bits(mods).unwrap(), key)
//         }
//     }
// }
// pub use bevy_mod_pkey as pkey;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bevy_key() {
        assert_eq!(keyseq_macros::bevy_pkey_u8!(shift-A), (1, KeyCode::A));
        // Can't do this here.
        // assert_eq!(pkey!(shift-A), (Modifiers::SHIFT, KeyCode::A));
    }
}
