#[cfg(feature = "bevy")]
mod for_bevy {
    use ::bevy::prelude::*;
    use keyseq::{bevy::*, Modifiers};

    #[test]
    fn check_modifiers() {
        assert_eq!(pkey! { shift-A }, (Modifiers::SHIFT, KeyCode::A));
        assert_eq!(pkey! { shift-B }, (Modifiers::SHIFT, KeyCode::B));
    }

    #[test]
    fn check_match() {
        match pkey! { ctrl-shift-A } {
            (Modifiers(5), KeyCode::A) => {}
            pkey! { ctrl-A } => panic!(),
            // pkey!(ctrl-shift-A) => {},
            _ => panic!(),
        }
    }
}
