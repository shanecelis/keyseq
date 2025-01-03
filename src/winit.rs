//! keyseq macros for winit library
use crate::Modifiers;
use ::winit::keyboard::ModifiersState;

/// Short hand notation describes a logical key chord as `(modifiers:`
/// [Modifiers]`, key: `[winit::keyboard::Key][key]`)`.
///
/// [key]: https://docs.rs/winit/latest/winit/keyboard/enum.Key.html
/// ```
/// use keyseq::{Modifiers, winit::lkey as key};
/// use winit::keyboard::Key;
///
/// assert_eq!(key! { a },          (Modifiers::NONE,    Key::Character('a')));
/// assert_eq!(key! { Ctrl-a },     (Modifiers::CONTROL, Key::Character('a')));
/// assert_eq!(key! { Alt-a },      (Modifiers::ALT,     Key::Character('a')));
/// assert_eq!(key! { Shift-a },    (Modifiers::SHIFT,   Key::Character('a')));
/// assert_eq!(key! { Super-a },    (Modifiers::SUPER,   Key::Character('a')));
/// assert_eq!(key! { Ctrl-Alt-; }, (Modifiers::ALT |
///                                 Modifiers::CONTROL, Key::Character(';')));
/// ```
///
/// This does have a limitation though because the macro does not do reverse look
/// ups from character to name.
///
/// ```compile_fail
/// # use keyseq::{Modifiers, winit::lkey};
/// use winit::keyboard::Key;
/// assert_eq!(lkey! { Ctrl-Semicolon }, (Modifiers::CONTROL, Key::Character(';')));
/// ```
pub use keyseq_macros::winit_lkey as lkey;

/// Short hand notation describes a sequence of logical key chords as `[(modifiers:
/// `[Modifiers]`, key: `[winit::keyboard::Key][key]`)]`.
///
/// [key]: https://docs.rs/winit/latest/winit/keyboard/enum.Key.html
pub use keyseq_macros::winit_lkeyseq as lkeyseq;

/// Short hand notation describes a physical key chord as `(modifiers:`
/// [Modifiers]`, key_code: `[winit::keyboard::KeyCode][keycode]`)`.
///
/// [keycode]: https://docs.rs/winit/latest/winit/keyboard/enum.KeyCode.html
/// ```
/// use keyseq::{Modifiers, winit::pkey as pkey};
/// use winit::keyboard::KeyCode;
/// assert_eq!(pkey! { A }, (Modifiers::NONE, KeyCode::KeyA));
/// ```
pub use keyseq_macros::winit_pkey as pkey;
/// Short hand notation describes a sequence of physical key chords as `[(modifiers:`
/// [Modifiers]`, key_code: `[winit::keyboard::KeyCode][keycode]`)]`.
///
/// [keycode]: https://docs.rs/winit/latest/winit/keyboard/enum.KeyCode.html
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
