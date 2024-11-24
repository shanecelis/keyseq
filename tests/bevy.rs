#[cfg(feature = "bevy")]
mod for_bevy {
    use ::bevy::{prelude::*, input::keyboard::Key};
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

    #[test]
    fn pkey0() {
        assert_eq!(pkeyseq! { Ctrl-Semicolon }, [(Modifiers::CONTROL, KeyCode::Semicolon)]);
    }

    #[test]
    fn lkeyseq0() {
        assert_eq!(lkeyseq! { Ctrl-; }, [(Modifiers::CONTROL, Key::Character(";".into()))]);
        assert_eq!(lkeyseq! { Ctrl-: }, [(Modifiers::CONTROL, Key::Character(":".into()))]);
        // TODO: Is this ok? Isn't there an implicit shift? Does it matter?
        assert_eq!(lkeyseq! { Ctrl-A }, [(Modifiers::CONTROL, Key::Character("A".into()))]);
        assert_eq!(lkeyseq! { Ctrl-a }, [(Modifiers::CONTROL, Key::Character("a".into()))]);
    }

    #[test]
    fn lkey0() {
        assert_eq!(lkey! { Ctrl-; }, (Modifiers::CONTROL, Key::Character(";".into())));
        assert_eq!(lkey! { Ctrl-: }, (Modifiers::CONTROL, Key::Character(":".into())));
        // TODO: Is this ok? Isn't there an implicit shift? Does it matter?
        assert_eq!(lkey! { Ctrl-A }, (Modifiers::CONTROL, Key::Character("A".into())));
        assert_eq!(lkey! { Ctrl-a }, (Modifiers::CONTROL, Key::Character("a".into())));
    }

    // TODO: These doc tests don't work.
    // /// ```
    // /// assert!(false);
    // /// assert_eq!(lkeyseq! { Ctrl-Semicolon }, [(Modifiers::CONTROL, Key::Character(";".into()))]);
    // /// ````
    // struct TestLkey1;

}
