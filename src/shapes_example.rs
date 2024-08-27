use std::default;

use freetype::Face;
use glfw::{ffi::glfwGetTime, Context};

use nalgebra_glm::{Vec2, Vec3, Vec4};
use plot_rs::{
    buffers::{ArrayBuffer, GLAttrib},
    camera::{self, Camera, CameraProjection, CameraView},
    coord2d::{ArrowLocation, XYCoordinates},
    coordinate_mapping::Coordinate,
    shader_builder::{shader::CompileProgram, RawShader},
    shapes::{
        self,
        Shapes::{
            self, combine_triangles_level0, DynamicLineFunction, Quad, Shape, ShapeDrawCall,
            ShapeVertexType,
        },
        Surfaces, TwoDim,
    },
    texts::{TextBoxes, TextGenerator},
    textures::Texture,
    window::Window,
};
use Shapes::{combine_triangles_level1, Circle, DrawableShape};

fn main() {
    let mut window = Window::default();

    unsafe {
        gl::Disable(gl::CULL_FACE);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::DepthMask(gl::TRUE);
    }

    let program = CompileProgram(
        "shaders/shapes/vertex.glsl",
        "shaders/shapes/fragment.glsl",
        None,
    );

    let text_program = CompileProgram(
        "shaders/textures/vertex.glsl",
        "shaders/textures/fragment.glsl",
        None,
    );

    let mut cam = Camera::new(
        CameraProjection::default_perspective(1.0, 1.5),
        CameraView::default(),
    );

    let freetype_lib = freetype::Library::init().unwrap();
    let font = freetype_lib.new_face("cambriaz.ttf", 0).unwrap();

    let mut texts = TextBoxes::new(text_program, font, 300);
    texts.add_text("Hello World", 1.0, Coordinate::default());
    // texts.add_text("Hello World", 5.0, 100, "cambriaz.ttf", None, None);
    texts.update_array_buffer();

    texts.add_camera(&mut cam);

    while !window.m_window.should_close() {
        window.m_glfw.poll_events();

        for (_, event) in glfw::flush_messages(&window.m_receiver) {
            Camera::update_camera(&event, &mut cam);
        }
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            texts.draw_textures();
        }
        window.m_window.swap_buffers();
    }
}

// TODO Add more shapes
fn main_dynamic_shapes() {
    let mut window = Window::default();

    unsafe {
        gl::Disable(gl::CULL_FACE);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::DepthMask(gl::TRUE);
    }

    let coords = XYCoordinates::builder()
        .x_range((-5.0, 5.0))
        .y_range((-5.0, 5.0))
        .xy_subgrid((3, 3))
        .density((11, 11))
        .line_width(0.01)
        .seco_color(Vec4::new(1.0, 1.0, 1.0, 0.2))
        .arrow_specification((0.1, 0.02, 0.16))
        .arrow_location(ArrowLocation::Center)
        .arrow_color(Vec4::new(1.0, 1.0, 1.0, 0.9))
        .build()
        .generate_shape();

    unsafe { gl::LineWidth(3.0) };
    let mut line_function = DynamicLineFunction::new(
        |t| Vec4::new(t.cos(), (0.37 * t).sin(), f32::sin(10.0 * f32::cos(t)), 1.0),
        0.0,
        6.0 * 3.141592653,
        10000,
        |t| {
            Vec4::new(
                0.5 + 0.5 * f32::sin(t),
                0.5 + 0.5 * f32::cos(t),
                0.75 + 0.25 * f32::cos(t),
                1.0,
            )
        },
    );

    let mut surface_function = Surfaces::ParametricSurface::new(
        |u, v| Vec4::new(u, v, (-u.powi(2) - v.powi(2)).exp(), 1.0),
        (-2.0, 2.0),
        (-2.0, 2.0),
        (1000, 1000),
        |u, v| {
            Vec4::new(
                1.0 + 0.5 * f32::sin(u),
                1.0 + 0.5 * f32::sin(v),
                1.0 + 0.5 * f32::sin(v + u),
                1.0 - (u.powi(2) + v.powi(2)).sqrt() / 2.0,
            )
        },
    );

    let lib = freetype::Library::init().unwrap();
    let face = lib.new_face("cambriaz.ttf", 0).unwrap();
    let mut tg = TextGenerator::new(&face, 100);
    let img = tg.generate_text_texture("Dynamic Function");

    let mut cam = Camera::default();
    cam.set_perspective(1.0, 1.5, 0.01, 100.0);
    cam.get_camera_view()
        .set_eye(&Vec3::new(2.0, 2.0, 2.0))
        .update_matrix();

    let program = CompileProgram(
        "shaders/shapes/vertex.glsl",
        "shaders/shapes/fragment.glsl",
        None,
    );
    line_function.bind_camera(&mut cam);
    surface_function.bind_camera(&mut cam);
    cam.add_program(program).update_uniforms(None);
    let shapesb = coords;

    //  Quad::new_from_two_points(
    //     &Vec2::new(0.0, -0.50),
    //     &Vec2::new(0.0, 0.50),
    //     0.10,
    //     &Vec3::new(1.0, 0.5, 0.35),
    // )
    // .generate_shape(0);

    let shapes = combine_triangles_level1([shapesb].to_vec());

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

    let t0 = unsafe { glfwGetTime() };
    while !window.m_window.should_close() {
        window.m_glfw.poll_events();
        for (_, event) in glfw::flush_messages(&window.m_receiver) {
            Camera::update_camera(&event, &mut cam);
        }
        unsafe {
            let tm = (glfwGetTime() - t0) as f32;
            line_function.update_line_inplace(
                |t| {
                    Vec4::new(
                        f32::cos(0.1 * tm + t),
                        f32::cos(0.314 * tm + t),
                        f32::cos(0.6 * tm * t),
                        1.0,
                    )
                },
                0.0,
                2.0 * 3.1415926,
            );
            surface_function.update_surface_inplace(
                |u, v| {
                    Vec4::new(
                        u,
                        v,
                        0.5 + {
                            let r = (u.powi(2) + v.powi(2)).sqrt();
                            f32::sin((2.0 + tm) * r).powi(2) * f32::exp(-2.1 * r)
                        },
                        1.0,
                    )
                },
                (-1.0, 1.0),
                (-1.0, 1.0),
            );
        }
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UseProgram(program);
            shapes_buffer.vao.bind();
            shapes_buffer.vbo.bind();
            shapes.draw();
            gl::DepthMask(gl::FALSE);
            line_function.draw_on(2.0);
            surface_function.draw_on();
            gl::DepthMask(gl::TRUE);
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
