//! My crate opengl_rs is awesome

extern crate gl;

use std::f32::consts::PI;
use cgmath::Vector3;
use cgmath::{prelude::*, Matrix4, Vector2};
use luve_rust::{circle::Circle, mesh::Object2D, shader::Shader, window::Window as RWindow};

pub fn main() {

    let mut my_window = RWindow::new(800, 600, "Luve Rust");

    // Load and compile our shader 🥰
    let shader = match Shader::new("./shaders/simple.vert", "./shaders/simple.frag") {
        Ok(e) => e,
        Err(e) => panic!("{}", e.message),
    };

    let circle_mesh = Circle::new(64, 400.0).build_mesh();

    let circle_1 = Circle::new(64, 100.0).build_mesh();

    my_window.start_loop(|my_window| {

        shader.use_it();
        
        let current_time = my_window.glfw.get_time() as f32;
        let (width, height) = my_window.raw_window.get_size();
        let (fwidth, fheight) = my_window.raw_window.get_framebuffer_size();

        let projection_matrix = cgmath::ortho(
            -width  as f32,
             width  as f32,
            -height as f32,
             height as f32,
            -10.0,
             10.0,
        );
        
        shader.set_matrix4("projection", &projection_matrix, false);
        shader.set_float("blendForce", 3.25);
        shader.set_float("iTime", current_time as f32);
        shader.set_vec2(
            "iResolution",
            &[Vector2::<f32>::new(fwidth as f32, fheight as f32)],
        );

        let mut transform_matrix_1 = Matrix4::<f32>::identity();
        shader.set_matrix4("transform", &transform_matrix_1, false);
        circle_mesh.draw();

        let mut x = (current_time + PI / 2.).cos() * 500.;
        let mut y = (current_time + PI / 2.).sin() * 500.;

        transform_matrix_1 = Matrix4::from_translation(Vector3::new(x, y, 0.));
        shader.set_matrix4("transform", &transform_matrix_1, false);
        circle_1.draw();

        x = (current_time + PI * 2.).cos() * 500.;
        y = (current_time + PI * 2.).sin() * 500.;

        transform_matrix_1 = Matrix4::from_translation(Vector3::new(x, y, 0.));
        shader.set_matrix4("transform", &transform_matrix_1, false);
        circle_1.draw();

        x = (current_time + PI).cos() * 500.;
        y = (current_time + PI).sin() * 500.;

        transform_matrix_1 = Matrix4::from_translation(Vector3::new(x, y, 0.));
        shader.set_matrix4("transform", &transform_matrix_1, false);
        circle_1.draw();

        x = (current_time + -PI / 2.).cos() * 500.;
        y = (current_time + -PI / 2.).sin() * 500.;

        transform_matrix_1 = Matrix4::from_translation(Vector3::new(x, y, 0.));
        shader.set_matrix4("transform", &transform_matrix_1, false);
        circle_1.draw();
    });
}