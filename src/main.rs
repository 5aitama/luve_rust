//! My crate opengl_rs is awesome

extern crate gl;
extern crate glfw;

mod mesh;
mod vertex;
mod shader;

use self::glfw::{ Context, Key, Action };
use cgmath::{ Vector2, Vector3, Matrix4, prelude::*};

use mesh::Mesh;
use vertex::Vertex;
use shader::Shader;

use std::sync::mpsc::Receiver;

const WINDOW_W: u32 = 800;
const WINDOW_H: u32 = 600;

pub fn main() {
    
    // Initialize GLFW...
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    
    // Add forward compatible hint for MacOS ...
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // Create our window...
    let (mut window, events) = glfw.create_window(WINDOW_W, WINDOW_H, "Luve Rust", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW Window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // Load OpenGL methods...
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Load and compile our shader 🥰
    let vs = "shaders/simple.vert";
    let fs = "shaders/simple.frag";
    let shader = match Shader::new(vs, fs) {
        Ok(e)   => e,
        Err(e)  => panic!("{}", e.message),
    };

    // Create a full screen quad (FSQ)
    // The vertices and uvs of our fsq...
    let vertices: Vec<Vertex<f32, f32>> = [
        Vertex::new(Vector3::new(-1.0, -1.0, 0.0), Vector2::new(0.0, 0.0)),
        Vertex::new(Vector3::new(-1.0,  1.0, 0.0), Vector2::new(0.0, 1.0)),
        Vertex::new(Vector3::new( 1.0,  1.0, 0.0), Vector2::new(1.0, 1.0)),
        Vertex::new(Vector3::new( 1.0, -1.0, 0.0), Vector2::new(1.0, 0.0)),
    ].to_vec();

    // The indices of our fsq geometry.
    let indices: Vec<Vector3<u8>> = [
        Vector3::new(0, 1, 2),
        Vector3::new(0, 2, 3),
    ].to_vec();
    
    // Build our fsq mesh from the vertices and indices...
    let mesh = Mesh::<f32, f32, u8>::new(vertices, indices, false);

    // Render loop...
    while !window.should_close() {
        // Process events, inputs, etc...
        process_events(&mut window, &events);

        let current_time = glfw.get_time() as f32;

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.1, 0.1, 0.1, 1.);

            gl::UseProgram(shader.get_program());

            let (width, height) = window.get_framebuffer_size();

            let projection_matrix = Matrix4::<f32>::identity(); //cgmath::ortho(-width as f32, width as f32, -height as f32, height as f32, -10.0, 10.0);
            shader.set_matrix4("projection", &projection_matrix, false);

            let transform_matrix = Matrix4::<f32>::identity();
            // transform_matrix = transform_matrix * Matrix4::<f32>::from_translation(vec3(-0.5, 0.0, 0.0));
            // transform_matrix = transform_matrix * Matrix4::<f32>::from_angle_z(cgmath::Rad(current_time));
            shader.set_matrix4("transform", &transform_matrix, false);

            shader.set_float("blendForce", 3.25);
            shader.set_float("iTime", current_time as f32);
            shader.set_vec2("iResolution", &[Vector2::<f32>::new(width as f32, height as f32)]);

            // Render our FSQ to the screen !
            mesh.draw();
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for(_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => unsafe {  gl::Viewport(0, 0, width, height); }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}
