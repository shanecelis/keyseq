//! keyseq macros for winit library
use ::winit::keyboard::ModifiersState;
use crate::Modifiers;

/// Short hand notation describes a logical key chord as `(modifiers:`
/// [winit::keyboard::ModifiersState][mods]`, key: `[winit::keyboard::Key][key]`)`.
///
/// [mods]: https://docs.rs/winit/latest/winit/keyboard/struct.ModifiersState.html
/// [key]: https://docs.rs/winit/latest/winit/keyboard/enum.Key.html
/// ```
/// use keyseq::{Modifiers, winit::key};
/// use winit::keyboard::Key;
///
/// assert_eq!(key!{ a },          (Modifiers::NONE,    Key::Character('a')));
/// assert_eq!(key!{ ctrl-a },     (Modifiers::CONTROL, Key::Character('a')));
/// assert_eq!(key!{ alt-a },      (Modifiers::ALT,     Key::Character('a')));
/// assert_eq!(key!{ shift-a },    (Modifiers::SHIFT,   Key::Character('a')));
/// assert_eq!(key!{ super-a },    (Modifiers::SUPER,   Key::Character('a')));
/// assert_eq!(key!{ ctrl-alt-; }, (Modifiers::ALT |
///                                 Modifiers::CONTROL, Key::Character(';')));
/// ```
///
/// This does have a limitation though because the macro does not do reverse look
/// ups from character to name.
///
/// ```compile_fail
/// # use keyseq_macros::winit_key as key;
/// use winit::keyboard::{ModifiersState, Key};
/// assert_eq!(key!(ctrl-Semicolon), (ModifiersState::CONTROL, Key::Character(';'))); // fails
/// ```
pub use keyseq_macros::winit_key as key;
pub use keyseq_macros::winit_keyseq as keyseq;

/// Short hand notation describes a physical key chord as `(modifiers:`
/// [winit::keyboard::ModifiersState][mods]`, key_code: `[winit::keyboard::KeyCode][keycode]`)`.
///
/// [mods]: https://docs.rs/winit/latest/winit/keyboard/struct.ModifiersState.html
/// [keycode]: https://docs.rs/winit/latest/winit/keyboard/enum.KeyCode.html
/// ```
/// use keyseq::{Modifiers, winit::pkey as pkey};
/// use winit::keyboard::KeyCode;
/// assert_eq!(pkey!(A), (Modifiers::NONE, KeyCode::KeyA));
/// ```
pub use keyseq_macros::winit_pkey as pkey;
pub use keyseq_macros::winit_pkeyseq as pkeyseq;

impl From<ModifiersState> for Modifiers {
    fn from(mods: ModifiersState) -> Self {
        let mut r = Modifiers::NONE;
        if mods.contains(ModifiersState::SHIFT) {
            r |= Modifiers::SHIFT;
        }
        if mods.contains(ModifiersState::CONTROL) {
            r |= Modifiers::CONTROL;
        }
        if mods.contains(ModifiersState::ALT) {
            r |= Modifiers::ALT;
        }
        if mods.contains(ModifiersState::SUPER) {
            r |= Modifiers::SUPER;
        }
        r
    }
}

impl From<Modifiers> for ModifiersState {
    fn from(mods: Modifiers) -> Self {
        let mut r = ModifiersState::empty();
        if mods.contains(Modifiers::CONTROL) {
            r |= ModifiersState::CONTROL;
        }
        if mods.contains(Modifiers::ALT) {
            r |= ModifiersState::ALT;
        }
        if mods.contains(Modifiers::SHIFT) {
            r |= ModifiersState::SHIFT;
        }
        if mods.contains(Modifiers::SUPER) {
            r |= ModifiersState::SUPER;
        }
        r
    }
}
