extern crate rustils;

use std::thread;
use std::sync::{Arc, Mutex};
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use game_loop::game_loop;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};
use crate::core::Core;
use crate::core::register_type::RegisterType8;
// use game_loop::game_loop;

mod core;

fn main() {
    let mut core = Core::new();

    let mut file = File::open("/Users/hevey/Development/PlayCade/gb-test-roms/cpu_instrs/individual/06-ld r,r.gb").expect("No file found");
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).expect("Failed to open file");

    core.cpu.memory.write_game(&bytes[..]);
    core.cpu.memory.write(0xFF44, 0x90);
    core.cpu.register.write_8(RegisterType8::F, 0xB0);
    //
    // let event_loop = EventLoop::new();
    // let window = WindowBuilder::new().build(&event_loop).unwrap();
    //
    // event_loop.run(move |event, _, control_flow| {
    //     // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    //     // dispatched any events. This is ideal for games and similar applications.
    //     control_flow.set_poll();
    //
    //     // ControlFlow::Wait pauses the event loop if no events are available to process.
    //     // This is ideal for non-game applications that only update in response to user
    //     // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    //     control_flow.set_wait();
    //
    //     match event {
    //         Event::WindowEvent {
    //             event: WindowEvent::CloseRequested,
    //             ..
    //         } => {
    //             println!("The close button was pressed; stopping");
    //             control_flow.set_exit();
    //         },
    //         Event::MainEventsCleared => {
    //             // Application update code.
    //
    //             // Queue a RedrawRequested event.
    //             //
    //             // You only need to call this if you've determined that you need to redraw, in
    //             // applications which do not always need to. Applications that redraw continuously
    //             // can just render here instead.
    //             window.request_redraw();
    //         },
    //         Event::RedrawRequested(_) => {
    //             // Redraw the application.
    //             //
    //             // It's preferable for applications that do not render continuously to render in
    //             // this event rather than in MainEventsCleared, since rendering in here allows
    //             // the program to gracefully handle redraws requested by the OS.
    //         },
    //         _ => ()
    //     }
    // });


    game_loop(core, 1 * 4194304, 1f64 / 4194304f64, |g| {
        g.game.cpu.tick();
    }, |g| { });

    println!("Game Loaded");


}