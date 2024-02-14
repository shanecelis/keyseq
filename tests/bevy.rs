use ::bevy::prelude::*;
use keyseq::{Modifiers, bevy::*};

#[test]
fn check_modifiers() {
    assert_eq!(pkey!(shift-A), (Modifiers::SHIFT, KeyCode::A));
    assert_eq!(pkey!(shift-B), (Modifiers::SHIFT, KeyCode::B));
}
