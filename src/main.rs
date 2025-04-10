use std::time::{Duration, Instant};

use ::tracing::info;
/// Main binary function, responsible for:
/// * Calling command line parsing logic
/// * Setting up configuration
/// * Calling the run function in lib.rs
/// * Handling the error if the above returns an error

use winit::event::WindowEvent;
use winit::event_loop::{ControlFlow, EventLoop};

mod renderer;
use winit::window::Window;

// pub fn main() {
//     rust_template::run();
// }

use winit::{application::ApplicationHandler, event_loop::ActiveEventLoop};
#[cfg(web_platform)]
use winit::platform::web::WindowAttributesExtWeb;
use winit::window::WindowId;

#[path = "util/fill.rs"]
mod fill;
#[path = "util/tracing.rs"]
mod tracing;

struct App {
    window: Option<Window>,
    start_time: Instant,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
        self.start_time = Instant::now();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                // println!("The close button was pressed; stopping");
                info!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // fill::fill_window(&self.window.as_ref().unwrap());
                // let mut render = Renderer::new(800, 600);
                // let new_buffer = fill::fill_window();
                // render.set_buffer(new_buffer);
                // render.render();
                if let Some(window) = &self.window {
                    // Call the animated color function
                    // fill::fill_window_with_animated_color(window, self.start_time);
                    renderer::render(window);
                }


                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() {

    tracing::init();

    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    // event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App {
        window: None,
        start_time: Instant::now(),
    };
    let _ = event_loop.run_app(&mut app);
}