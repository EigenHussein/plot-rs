use glfw::Context;

use nalgebra_glm::{Vec2, Vec3, Vec4};
use plot_rs::{
    buffers::{ArrayBuffer, GLAttrib},
    camera::Camera,
    shader_builder::shader::CompileProgram,
    shapes::{
        Shapes::{self, combine_triangles_level0, Shape},
        TwoDim,
    },
    window::Window,
};
use Shapes::{combine_triangles_level1, Circle, DrawableShape};
// TODO Add more shapes

fn generate_shape() -> Shape {
    let s1 = Shapes::Quad::new_from_two_points(
        &Vec2::zeros(),
        &Vec2::x(),
        0.1,
        &Vec3::new(0.1, 0.4, 0.8),
    )
    .generate_shape(0);
    let s2 =
        Shapes::Circle::new_circle(Vec3::zeros(), 1.0, Some(Vec3::x()), Some(20)).generate_shape(0);

    combine_triangles_level1([s1, s2].to_vec())
}
fn generate_arrow(p1: Vec2, p2: Vec2, head_percentage: f32, w1: f32, w2: f32) -> Shape {
    let dir = p2 - p1;
    let p1h = p1 + (1.0 - head_percentage) * dir;
    let quad = Shapes::Quad::new_from_two_points(&p1, &p1h, w1, &Vec3::new(0.0, 0.5, 0.25))
        .generate_shape(0);
    let head = Shapes::Triangle::new(p1h, p2, w2, Some(Vec3::new(0.0, 0.9, 0.8))).generate_shape(0);
    combine_triangles_level1([quad, head].to_vec())
}
fn main() {
    let mut window = Window::default();

    let mut cam = Camera::default();
    cam.set_perspective(1.0, 1.5, 0.01, 100.0);
    let program = CompileProgram(
        "shaders/shapes/vertex.glsl",
        "shaders/shapes/fragment.glsl",
        None,
    );
    cam.add_program(program).update_uniforms(None);

    let shapes = generate_arrow(Vec2::zeros(), Vec2::y(), 0.1, 0.01, 0.02); //generate_shape();
    let vertex_count = shapes.vertices.len().clone();
    println!("{:?} {}", shapes.draw_calls, vertex_count);
    let mut shapes_buffer = ArrayBuffer::default_buffer_from_slice(
        &shapes.vertices,
        None,
        &[
            GLAttrib::new(0, 3, 8, 0),
            GLAttrib::new(1, 3, 8, 3),
            GLAttrib::new(2, 2, 8, 6),
        ]
        .to_vec(),
        true,
    );

    while !window.m_window.should_close() {
        window.m_glfw.poll_events();
        for (_, event) in glfw::flush_messages(&window.m_receiver) {
            Camera::update_camera(&event, &mut cam);
        }
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UseProgram(program);
            shapes_buffer.vao.bind();
            shapes_buffer.vbo.bind();
            shapes.draw();
        }
        window.m_window.swap_buffers();
    }
}

fn main_testing_combinations() {
    let quad1 = &Shapes::Quad::new_from_two_points(
        &Vec2::new(0.0, 0.0),
        &Vec2::new(0.0, 1.0),
        0.5,
        &Vec3::new(1.0, 0.5, 0.3),
    )
    .generate_shape(0);
    let circle = &Shapes::Circle::new_circle(Vec3::zeros(), 1.0, None, None).generate_shape(0);
    let quad2 = &Shapes::Quad::new_from_two_points(
        &Vec2::new(0.0, 0.0),
        &Vec2::new(1.0, 1.0),
        0.5,
        &Vec3::new(1.0, 0.5, 0.3),
    )
    .generate_shape(0);
    let circle2 = &Shapes::Circle::new_circle(Vec3::zeros(), 1.0, None, Some(20)).generate_shape(0);

    let shape = combine_triangles_level1(vec![
        quad1.clone(),
        circle.clone(),
        circle2.clone(),
        quad2.clone(),
    ]);
    println!("{:?}", shape.draw_calls);
}

fn main_showing_simpe_shape() {
    let mut window = Window::default();

    let circle = Shapes::Quad::new_from_two_points(
        &Default::default(),
        &Vec2::new(1.0, 1.0),
        5.0,
        &Vec3::new(1.0, 0.5, 0.25),
    );

    let circle_info = &circle.generate_shape(0);

    let mut ab = ArrayBuffer::default_buffer_from_slice(
        &circle_info.vertices,
        None,
        &[GLAttrib::new(0, 3, 8, 0)].to_vec(),
        true,
    );

    let mut program = CompileProgram(
        "/home/hussein/Dev/Rust/plotters/shaders/shapes/vertex.glsl",
        "/home/hussein/Dev/Rust/plotters/shaders/shapes/fragment.glsl",
        None,
    );

    let mut cam = Camera::default();
    cam.set_perspective(1.0, 1.5, 0.01, 100.0)
        .add_program(program)
        .update_uniforms(None);

    while !window.m_window.should_close() {
        window.m_glfw.poll_events();
        for (_, event) in glfw::flush_messages(&window.m_receiver) {
            Camera::update_camera(&event, &mut cam);
        }
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UseProgram(program);
            ab.vao.bind();
            ab.vbo.bind();
            circle_info.draw();
            // gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
        window.m_window.swap_buffers();
    }
}

fn main_twodim_quad() {
    let mut window = Window::default();

    let line = TwoDim::Quad::new_from_2points(&Vec3::zeros(), &Vec3::x(), 0.20, Some(&Vec3::z()));
    let mut ab = ArrayBuffer::default_buffer_from_slice(
        &line,
        None,
        &[GLAttrib::new(0, 3, 3, 0)].to_vec(),
        true,
    );

    let mut program = CompileProgram(
        "/home/hussein/Dev/Rust/plotters/shaders/shapes/vertex.glsl",
        "/home/hussein/Dev/Rust/plotters/shaders/shapes/fragment.glsl",
        None,
    );

    let mut cam = Camera::default();
    cam.set_perspective(1.0, 1.5, 0.01, 100.0)
        .add_program(program)
        .update_uniforms(None);

    while !window.m_window.should_close() {
        window.m_glfw.poll_events();
        for (_, event) in glfw::flush_messages(&window.m_receiver) {
            Camera::update_camera(&event, &mut cam);
        }
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UseProgram(program);
            ab.vao.bind();
            ab.vbo.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
        window.m_window.swap_buffers();
    }
}
