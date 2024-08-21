use std::default;

use glfw::Context;

use nalgebra_glm::{Vec2, Vec3, Vec4};
use plot_rs::{
    buffers::{ArrayBuffer, GLAttrib}, camera::Camera, coord2d::{ArrowLocation, XYCoordinates}, shader_builder::shader::CompileProgram, shapes::{
        self,
        Shapes::{self, combine_triangles_level0, Quad, Shape, ShapeVertexType},
        TwoDim,
    }, window::Window
};
use Shapes::{combine_triangles_level1, Circle, DrawableShape};
// TODO Add more shapes



fn main() {
    let mut window = Window::default();
    unsafe {
        gl::Disable(gl::CULL_FACE);
    }

    let coords = XYCoordinates::builder()
        .x_range((-5.0, 5.0))
        .y_range((-5.0, 5.0))
        .xy_subgrid((3, 3))
        .density((11, 11))
        .line_width(0.01)
        .seco_color(Vec4::new(1.0, 1.0, 1.0, 0.2))
        .arrow_specification((0.1, 0.02, 0.04))
        .arrow_location(ArrowLocation::Center)
        .arrow_color(Vec4::new(1.0, 1.0, 1.0, 0.9))
        .build()
        .generate_shape();

    for vert in coords.vertices.iter().enumerate() {
        println!("{:?}", vert);
    }

    let mut cam = Camera::default();
    cam.set_perspective(1.0, 1.5, 0.01, 100.0);
    let program = CompileProgram(
        "shaders/shapes/vertex.glsl",
        "shaders/shapes/fragment.glsl",
        None,
    );
    cam.add_program(program).update_uniforms(None);
    let shapes = coords;
    //  Quad::new_from_two_points(
    //     &Vec2::new(0.0, -0.50),
    //     &Vec2::new(0.0, 0.50),
    //     0.10,
    //     &Vec3::new(1.0, 0.5, 0.35),
    // )
    // .generate_shape(0);

    //let shapes = generate_arrow(Vec2::zeros(), Vec2::y(), 0.1, 0.01, 0.02); //generate_shape();
    let vertex_count = shapes.vertices.len().clone();
    println!("{:?} {}", shapes.draw_calls, vertex_count);
    let mut shapes_buffer = ArrayBuffer::default_buffer_from_slice(
        &shapes.vertices,
        None,
        &[
            GLAttrib::new(0, 4, 10, 0),
            GLAttrib::new(1, 4, 10, 4),
            GLAttrib::new(2, 2, 10, 8),
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
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
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
        &Vec4::new(1.0, 0.5, 0.3, 1.0),
        0.1,
    )
    .generate_shape(0);
    let circle = &Shapes::Circle::new_circle(Vec3::zeros(), 1.0, None, None).generate_shape(0);
    let quad2 = &Shapes::Quad::new_from_two_points(
        &Vec2::new(0.0, 0.0),
        &Vec2::new(1.0, 1.0),
        0.5,
        &Vec4::new(1.0, 0.5, 0.3, 1.0),
        0.2,
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
        &Vec4::new(1.0, 0.5, 0.25, 1.0),
        0.1,
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
