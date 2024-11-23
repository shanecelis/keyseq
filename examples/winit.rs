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
    window::Window,
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

    let attributes = Window::default_attributes()
        .with_inner_size(LogicalSize::new(400.0, 200.0));
    let _window = event_loop.create_window(attributes).unwrap();

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
                                pkey!{ Ctrl-A } | pkey!{ Super-A } => println!("Just pressed Ctrl-A or Super-A!"),
                                pkey!{ Ctrl-Alt-A }                => println!("Just pressed Ctrl-Alt-A!"),
                                pkey!{ Ctrl-Shift-A }              => println!("Just pressed Ctrl-Shift-A!"),
                                pkey!{ Alt-Shift-A }               => println!("Just pressed Alt-Shift-A!"),
                                pkey!{ Ctrl-Alt-Shift-A }          => println!("Just pressed Alt-Shift-A!"),
                                pkey!{ Shift-A }                   => println!("Just pressed Shift-A!"),
                                pkey!{ Alt-A }                     => println!("Just pressed Alt-A!"),
                                pkey!{ Super-A }                   => println!("Just pressed Super-A!"),
                                pkey!{ A }                         => println!("Just pressed A!"),
                                _                                  => println!("No key matched."),
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
