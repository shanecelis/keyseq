#[cfg(feature = "bevy")]
mod for_bevy {
    use ::bevy::prelude::*;
    use keyseq::{bevy::*, Modifiers};

    #[test]
    fn check_modifiers() {
        assert_eq!(pkey! { shift-A }, (Modifiers::SHIFT, KeyCode::KeyA));
        assert_eq!(pkey! { shift-B }, (Modifiers::SHIFT, KeyCode::KeyB));
    }

    #[test]
    fn check_match() {
        match pkey! { ctrl-shift-A } {
            (Modifiers(5), KeyCode::KeyA) => {}
            pkey! { ctrl-A } => panic!(),
            // pkey!(ctrl-shift-A) => {},
            _ => panic!(),
        }
    }
}
