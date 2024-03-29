//! keyseq macros for bevy game engine
use super::Modifiers;
use ::bevy::input::{keyboard::KeyCode, ButtonInput as Input};

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

/// Short hand notation describes a physical key chord as `(modifiers: `[Modifiers]`,
/// key_code: `[bevy::prelude::KeyCode][keycode]`)`.
///
/// Specify a key and any modifiers.
///
/// ```
/// use keyseq::{Modifiers, bevy::pkey};
/// use bevy::prelude::KeyCode;
/// assert_eq!(pkey! { A },          (Modifiers::NONE, KeyCode::KeyA));
/// assert_eq!(pkey! { ctrl-A },     (Modifiers::CONTROL, KeyCode::KeyA));
/// assert_eq!(pkey! { alt-A },      (Modifiers::ALT, KeyCode::KeyA));
/// assert_eq!(pkey! { shift-A },    (Modifiers::SHIFT, KeyCode::KeyA));
/// assert_eq!(pkey! { super-A },    (Modifiers::SUPER, KeyCode::KeyA));
/// assert_eq!(pkey! { ctrl-alt-; }, (Modifiers::CONTROL | Modifiers::ALT, KeyCode::Semicolon));
/// assert_eq!(pkey! { 1 },          (Modifiers::NONE, KeyCode::Digit1));
/// assert_eq!(pkey! { alt-1 },      (Modifiers::ALT, KeyCode::Digit1));
/// ```
///
/// More than one key will cause a panic at compile-time. Use keyseq! for that.
///
/// ```compile_fail
/// # use keyseq::bevy::pkey;
/// fn too_many_keys() {
///     let _ = pkey! { A B };
/// }
/// ```
///
/// This macro does not ensure the key names exist but the compiler will.
///
/// ```compile_fail
/// use keyseq::bevy::pkey;
/// use bevy::prelude::KeyCode;
/// let (mods, key) = pkey! { alt-NoSuchKey }; // KeyCode::NoSuchKey does not exist.
/// ```
/// [keycode]: https://docs.rs/bevy/latest/bevy/prelude/enum.KeyCode.html
pub use keyseq_macros::bevy_pkey as pkey;

/// Short hand notation describes a sequence of physical key chords as
/// `[(modifiers: `[Modifiers]`, key: `[bevy::prelude::KeyCode][keycode]`)]`.
///
/// [keycode]: https://docs.rs/bevy/latest/bevy/prelude/enum.KeyCode.html
pub use keyseq_macros::bevy_pkeyseq as pkeyseq;
