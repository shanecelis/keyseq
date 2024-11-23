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
}
