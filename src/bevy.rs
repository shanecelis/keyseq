//! keyseq macros for bevy game engine
use super::Modifiers;
use ::bevy::{
    input::{keyboard::KeyCode, ButtonInput},
    prelude::Res,
};

impl Modifiers {
    /// Check modifier keys for `any_pressed()` to populate bit flags.
    #[deprecated(since = "0.4.0", note = "Consider using `Modifiers::from()` instead.")]
    pub fn from_input(input: &ButtonInput<KeyCode>) -> Modifiers {
        Modifiers::from(input)
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

/// Check left and right modifier keys to populate bit flags.
impl From<&ButtonInput<KeyCode>> for Modifiers {
    #[inline(always)]
    fn from(input: &ButtonInput<KeyCode>) -> Self {
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

/// Convenience wrapper to avoid `Modifier::from(&*input)` shenanigans due to
/// resource type `Res<>` wrapper.
impl From<&Res<'_, ButtonInput<KeyCode>>> for Modifiers {
    #[inline(always)]
    fn from(input: &Res<ButtonInput<KeyCode>>) -> Self {
        <Modifiers as From<&ButtonInput<KeyCode>>>::from(input)
    }
}

/// Short hand notation describes a physical key chord as `(modifiers: `[Modifiers]`,
/// key_code: `[bevy::input::keyboard::KeyCode][keycode]`)`.
///
/// Specify a key and any modifiers.
///
/// ```
/// use keyseq::{Modifiers, bevy::pkey};
/// use bevy::input::keyboard::KeyCode;
/// assert_eq!(pkey! { A },          (Modifiers::NONE, KeyCode::KeyA));
/// assert_eq!(pkey! { Ctrl-A },     (Modifiers::CONTROL, KeyCode::KeyA));
/// assert_eq!(pkey! { Alt-A },      (Modifiers::ALT, KeyCode::KeyA));
/// assert_eq!(pkey! { Shift-A },    (Modifiers::SHIFT, KeyCode::KeyA));
/// assert_eq!(pkey! { Super-A },    (Modifiers::SUPER, KeyCode::KeyA));
/// assert_eq!(pkey! { Ctrl-Alt-; }, (Modifiers::CONTROL | Modifiers::ALT, KeyCode::Semicolon));
/// assert_eq!(pkey! { Ctrl-Alt-Semicolon }, (Modifiers::CONTROL | Modifiers::ALT, KeyCode::Semicolon));
/// assert_eq!(pkey! { 1 },          (Modifiers::NONE, KeyCode::Digit1));
/// assert_eq!(pkey! { Alt-1 },      (Modifiers::ALT, KeyCode::Digit1));
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
/// use bevy::input::keyboard::KeyCode;
/// let (mods, key) = pkey! { Alt-NoSuchKey }; // KeyCode::NoSuchKey does not exist.
/// ```
/// [keycode]: https://docs.rs/bevy/latest/bevy/prelude/enum.KeyCode.html
pub use keyseq_macros::bevy_pkey as pkey;

/// Short hand notation describes a sequence of physical key chords as
/// `[(modifiers: `[Modifiers]`, key: `[bevy::input::keyboard::KeyCode][keycode]`)]`.
///
/// [keycode]: https://docs.rs/bevy/latest/bevy/prelude/enum.KeyCode.html
pub use keyseq_macros::bevy_pkeyseq as pkeyseq;

/// Short hand notation describes a logical key chord as `(modifiers:`
/// [Modifiers]`, key: `[bevy::input::keyboard::Key][key]`)`.
///
/// [key]: https://docs.rs/bevy/latest/bevy/prelude/enum.Key.html
/// ```
/// use keyseq::{Modifiers, bevy::lkey as key};
/// use bevy::input::keyboard::Key;
///
/// assert_eq!(key!{ a },          (Modifiers::NONE,    Key::Character("a".into())));
/// assert_eq!(key!{ Ctrl-a },     (Modifiers::CONTROL, Key::Character("a".into())));
/// assert_eq!(key!{ Alt-a },      (Modifiers::ALT,     Key::Character("a".into())));
/// assert_eq!(key!{ Shift-a },    (Modifiers::SHIFT,   Key::Character("a".into())));
/// assert_eq!(key!{ Super-a },    (Modifiers::SUPER,   Key::Character("a".into())));
/// assert_eq!(key!{ Ctrl-Alt-; }, (Modifiers::ALT |
///                                 Modifiers::CONTROL, Key::Character(";".into())));
/// ```
///
/// This does have a limitation though because the macro does not do reverse look
/// ups from character to name.
///
/// ```compile_fail
/// # use keyseq::{Modifiers, bevy::lkey};
/// use bevy::input::keyboard::Key;
/// assert_eq!(lkey!(Ctrl-Semicolon), (Modifiers::CONTROL, Key::Character(";".into())));
/// ```
pub use keyseq_macros::bevy_lkey as lkey;

/// Short hand notation describes a sequence of logical key chords as `[(modifiers:
/// `[Modifiers]`, key: `[bevy::input::keyboard::Key][key]`)]`.
///
/// [key]: https://docs.rs/bevy/latest/bevy/prelude/enum.Key.html
pub use keyseq_macros::bevy_lkeyseq as lkeyseq;
