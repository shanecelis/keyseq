#![allow(clippy::single_match)]
use std::collections::HashMap;
#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::{Key, ModifiersState, PhysicalKey, KeyCode},
    // WARNING: This is not available on all platforms (for example on the web).
    platform::modifier_supplement::KeyEventExtModifierSupplement,
    window::{Window, WindowBuilder},
};
use keyseq::winit::{pkey, keyseq};

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
fn main() {
    println!("This example is not supported on this platform");
}

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
fn main() -> Result<(), impl std::error::Error> {
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(400.0, 200.0))
        .build(&event_loop)
        .unwrap();

    let mut modifiers = ModifiersState::default();
    let mut binds = HashMap::new();

    binds.insert(pkey! { 1 }, "1");
    binds.insert(pkey! { shift-1 }, "shift-1");

    event_loop.run(move |event, elwt| {
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::ModifiersChanged(new) => {
                    modifiers = new.state();
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    if event.state == ElementState::Pressed && !event.repeat {
                        // println!("Got key {:?}", event.logical_key);
                        if let PhysicalKey::Code(key_code) = event.physical_key {
                            if let Some(j) = binds.get(&(modifiers, key_code)) {
                                println!("Got key binding {:?}", j);
                            }
                        }
                    }
                }
                WindowEvent::RedrawRequested => {
                }
                _ => (),
            }
        };
    })
}