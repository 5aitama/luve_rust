extern crate gl;
extern crate glfw;

use std::sync::mpsc::Receiver;
use self::glfw::{Action, Context, Key, WindowEvent};

pub struct Window {
    /// The window width
    width: u32,

    /// The window height
    height: u32,

    /// The window title.
    title: String,

    /// The GLFW window that this struct
    /// wrapped.
    pub raw_window: glfw::Window,

    /// GLFW Events.
    events: Receiver<(f64, WindowEvent)>,

    /// The GLFW instance.
    pub glfw: glfw::Glfw,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Window {

        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        // Add forward compatible hint for MacOS ...
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        // Create our window...
        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW Window");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        
        // Load OpenGL methods...
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        Window {
            width: width,
            height: height,
            title: String::from(title),
            raw_window: window,
            events: events,
            glfw: glfw,
        }
    }

    pub fn start_loop<F: Fn(&mut Window) -> ()>(&mut self, loop_handle: F) {
        // Render loop...
        while !self.raw_window.should_close() {
            // Process events, inputs, etc...
            self.process_events();

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                gl::ClearColor(0.1, 0.1, 0.1, 1.);

                loop_handle(self);

                self.raw_window.swap_buffers();
                self.glfw.poll_events();
            }
        }
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height);
                },
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.raw_window.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}
