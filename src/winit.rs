//! keyseq macros for winit library
// use ::winit::keyboard::{ModifiersState, KeyCode};

/// Short hand notation describes a logical key chord as `(modifiers:`
/// [winit::keyboard::ModifiersState][mods]`, key: `[winit::keyboard::Key][key]`)`.
///
/// [mods]: https://docs.rs/winit/latest/winit/keyboard/struct.ModifiersState.html
/// [key]: https://docs.rs/winit/latest/winit/keyboard/enum.Key.html
/// ```
/// use keyseq_macros::winit_key as key;
/// use winit::keyboard::{ModifiersState, Key};
/// assert_eq!(key!(a), (ModifiersState::empty(), Key::Character('a')));
/// assert_eq!(key!(A), (ModifiersState::empty(), Key::Character('A')));
/// assert_eq!(key!(shift-A), (ModifiersState::SHIFT, Key::Character('A')));
/// assert_eq!(key!(shift-a), (ModifiersState::SHIFT, Key::Character('a')));
/// assert_eq!(key!(ctrl-A), (ModifiersState::CONTROL, Key::Character('A')));
/// assert_eq!(key!(alt-A), (ModifiersState::ALT, Key::Character('A')));
/// assert_eq!(key!(super-A), (ModifiersState::SUPER, Key::Character('A')));
/// assert_eq!(key!(ctrl-alt-;), (ModifiersState::ALT | ModifiersState::CONTROL, Key::Character(';')));
/// assert_eq!(key!(1), (ModifiersState::empty(), Key::Character('1')));
/// assert_eq!(key!(!), (ModifiersState::empty(), Key::Character('!')));
/// ```
///
/// ```
/// use keyseq_macros::winit_pkey as pkey;
/// use winit::keyboard::{ModifiersState, KeyCode};
/// assert_eq!(pkey!(A), (ModifiersState::empty(), KeyCode::KeyA));
/// ```
///
/// ```
/// use keyseq_macros::winit_key as key;
/// use winit::keyboard::{ModifiersState, Key};
/// assert_eq!(key!(;), (ModifiersState::empty(), Key::Character(';')));
/// assert_eq!(key!(ctrl-;), (ModifiersState::CONTROL, Key::Character(';')));
/// ```
///
/// This does have a limitation though because the macro does not do reverse look
/// ups from character to name.
///
/// ```compile_fail
/// # use keyseq_macros::winit_key as key;
/// use winit::keyboard::{ModifiersState, Key};
/// assert_eq!(key!(ctrl-Semicolon), (ModifiersState::CONTROL, Key::Character(';')));
/// ```
pub use keyseq_macros::winit_key as key;
pub use keyseq_macros::winit_keyseq as keyseq;

/// Short hand notation describes a physical key chord as `(modifiers:`
/// [winit::keyboard::ModifiersState][mods]`, key_code: `[winit::keyboard::KeyCode][keycode]`)`.
///
/// [mods]: https://docs.rs/winit/latest/winit/keyboard/struct.ModifiersState.html
/// [keycode]: https://docs.rs/winit/latest/winit/keyboard/enum.KeyCode.html
/// ```
/// use keyseq_macros::winit_pkey as pkey;
/// use winit::keyboard::{ModifiersState, KeyCode};
/// assert_eq!(pkey!(A), (ModifiersState::empty(), KeyCode::KeyA));
/// ```
pub use keyseq_macros::winit_pkey as pkey;
pub use keyseq_macros::winit_pkeyseq as pkeyseq;
