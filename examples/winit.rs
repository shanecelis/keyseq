#![allow(clippy::single_match)]
use keyseq::winit::pkey;
#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::{ModifiersState, PhysicalKey},
    // WARNING: This is not available on all platforms (for example on the web).
    // platform::modifier_supplement::KeyEventExtModifierSupplement,
    window::WindowBuilder,
};

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
fn main() {
    println!("This example is not supported on this platform");
}

#[rustfmt::skip]
#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
fn main() -> Result<(), impl std::error::Error> {
    println!("Press A key with different modifier keys.");
    let event_loop = EventLoop::new().unwrap();

    let _window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(400.0, 200.0))
        .build(&event_loop)
        .unwrap();

    let mut modifiers = ModifiersState::default();

    event_loop.run(move |event, elwt| {
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::ModifiersChanged(new) => {
                    modifiers = new.state();
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    if event.state == ElementState::Pressed && !event.repeat {
                        if let PhysicalKey::Code(key_code) = event.physical_key {
                            match (modifiers.into(), key_code) {
                                pkey!(ctrl-A) | pkey!(super-A) => println!("Just pressed ctrl-A or super-A!"),
                                pkey!(ctrl-alt-A)              => println!("Just pressed ctrl-alt-A!"),
                                pkey!(ctrl-shift-A)            => println!("Just pressed ctrl-shift-A!"),
                                pkey!(alt-shift-A)             => println!("Just pressed alt-shift-A!"),
                                pkey!(shift-A)                 => println!("Just pressed shift-A!"),
                                pkey!(alt-A)                   => println!("Just pressed alt-A!"),
                                pkey!(super-A)                 => println!("Just pressed super-A!"),
                                pkey!(A)                       => println!("Just pressed A!"),
                                _                              => println!("No key matched"),
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
