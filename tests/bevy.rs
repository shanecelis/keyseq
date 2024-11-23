#[cfg(feature = "bevy")]
mod for_bevy {
    use ::bevy::prelude::*;
    use keyseq::{bevy::*, Modifiers};

    #[test]
    fn check_modifiers() {
        assert_eq!(pkey! { Shift-A }, (Modifiers::SHIFT, KeyCode::KeyA));
        assert_eq!(pkey! { Shift-B }, (Modifiers::SHIFT, KeyCode::KeyB));
    }

    #[test]
    fn check_match() {
        match pkey! { Ctrl-Shift-A } {
            (Modifiers(5), KeyCode::KeyA) => {}
            pkey! { Ctrl-A } => panic!(),
            // pkey!(Ctrl-Shift-A) => {},
            _ => panic!(),
        }
    }

    #[test]
    fn check_display() {
        assert_eq!(pkey! { A }.0.to_string(), "");
        assert_eq!(pkey! { Shift-A }.0.to_string(), "Shift");
        assert_eq!(pkey! { Ctrl-Shift-A }.0.to_string(), "Ctrl-Shift");
        assert_eq!(pkey! { Ctrl-Alt-Shift-A }.0.to_string(), "Ctrl-Alt-Shift");
        assert_eq!(pkey! { Ctrl-Alt-Super-A }.0.to_string(), "Ctrl-Alt-Super");
    }
}
