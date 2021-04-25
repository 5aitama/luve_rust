extern crate gl;
extern crate glfw;

use std::path::Path;
use cgmath::vec3;
use cgmath::Matrix4;
use cgmath::{ Vector2, Vector3 };
use cgmath::prelude::*;
use gl::types::GLubyte;
use gl::types::GLsizei;
use gl::types::GLsizeiptr;
use gl::types::GLfloat;

use std::ptr;
use std::ffi::CString;
use core::ffi::c_void;

use self::glfw::{ Context, Key, Action };

mod shader;
use shader::Shader;

mod mesh;
use mesh::Mesh;

use std::sync::mpsc::Receiver;

const WINDOW_W: u32 = 800;
const WINDOW_H: u32 = 600;

pub fn main() {
    
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw.create_window(WINDOW_W, WINDOW_H, "my window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW Window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let vs = "shaders/simple.vert";
    let fs = "shaders/simple.frag";

    let shader = match Shader::new(vs, fs) {
        Ok(e)   => e,
        Err(e)  => panic!("{}", e.message),
    };

    // let m = Mesh::new([
    //     Vector3::<f32>::new(-0.5, -0.5, 0.0) * 250.0,
    //     Vector3::<f32>::new(-0.5,  0.5, 0.0) * 250.0,
    //     Vector3::<f32>::new( 0.5,  0.5, 0.0) * 250.0,
    //     Vector3::<f32>::new( 0.5, -0.5, 0.0) * 250.0,
    // ].to_vec(), [
    //     Vector2::<f32>::new(0.0, 0.0),
    //     Vector2::<f32>::new(0.0, 1.0),
    //     Vector2::<f32>::new(1.0, 1.0),
    //     Vector2::<f32>::new(1.0, 0.0),
    // ].to_vec(), [
    //     Vector3::<u8>::new(0, 1, 2),
    //     Vector3::<u8>::new(0, 2, 3),
    // ].to_vec(), false);

    let vao = unsafe {
        let vertices: [Vector3::<f32>; 4] = [
            Vector3::<f32>::new(-0.5, -0.5, 0.0) * 250.0,
            Vector3::<f32>::new(-0.5,  0.5, 0.0) * 250.0,
            Vector3::<f32>::new( 0.5,  0.5, 0.0) * 250.0,
            Vector3::<f32>::new( 0.5, -0.5, 0.0) * 250.0,
        ];

        let indices: [u8; 6] = [
            0, 1, 2,
            0, 2, 3,
        ];

        let (mut vbo, mut vao, mut ebo) = (0, 0, 0);

        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        let vsize = (vertices.len() * std::mem::size_of::<Vector3::<f32>>()) as GLsizeiptr;
        let isize = (indices.len() * std::mem::size_of::<GLubyte>()) as GLsizeiptr;

        gl::BufferData(gl::ARRAY_BUFFER, vsize, ptr::null(), gl::STATIC_DRAW);

        gl::BufferSubData(gl::ARRAY_BUFFER, 0, vsize, &vertices[0] as *const Vector3::<f32> as *const c_void);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, isize, &indices[0] as *const u8 as *const c_void, gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);


        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        vao
    };

    //let mut last_time = 0f32;

    while !window.should_close() {

        process_events(&mut window, &events);

        let current_time = glfw.get_time() as f32;

        // let delta_time = current_time - last_time;
        // last_time  = current_time;

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.1, 0.1, 0.1, 1.);

            gl::UseProgram(shader.get_program());

            let name = CString::new("time").unwrap();
            let location = gl::GetUniformLocation(shader.get_program(), name.as_ptr());
            gl::Uniform1f(location, current_time);

            let (width, height) = window.get_framebuffer_size();

            let projection_matrix = cgmath::ortho(-width as f32, width as f32, -height as f32, height as f32, -10.0, 10.0);
            shader.set_matrix4("projection", &projection_matrix, false);

            let mut transform_matrix = Matrix4::<f32>::identity();
            transform_matrix = transform_matrix * Matrix4::<f32>::from_translation(vec3(-0.5, 0.0, 0.0));
            transform_matrix = transform_matrix * Matrix4::<f32>::from_angle_z(cgmath::Rad(current_time));

            shader.set_matrix4("transform", &transform_matrix, false);
            // m.draw();
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_BYTE, ptr::null());
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for(_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => unsafe { 
                gl::Viewport(0, 0, width, height);
                
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}
