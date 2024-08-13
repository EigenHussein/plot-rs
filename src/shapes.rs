use glfw::Context;

use nalgebra_glm::Vec3;
use plot_rs::{
    buffers::{ArrayBuffer, GLAttrib},
    camera::Camera,
    shader_builder::shader::CompileProgram,
    shapes::TwoDim,
    window::Window,
};
// TODO Add more shapes
pub mod Shapes {

    type VertexType = (Vec3, Vec3, Vec2);
    use std::f32::consts::PI;

    use nalgebra_glm::{Mat4, Vec2, Vec3, Vec4};
    #[derive(Default)]
    pub struct Shape {
        /// ### Vertex | Color | TXCoord
        pub vertices: Vec<VertexType>,
        /// ### mode | offset | count
        pub draw_call: (u32, u32, u32),
    }

    trait DrawableShape {
        ///
        /// ### Description
        /// Generate vertices + draw call info
        fn generate_shape(&self, offset: u32) -> Vec<Shape>;
        // fn get_vertices_mut(&mut self) -> &Vec<VertexType>;
        fn generate_transformed_shape(
            &self,
            offset: u32,
            transformation_matrix: &Mat4,
        ) -> Vec<Shape> {
            let mut shapes = self.generate_shape(offset);

            for shape in shapes.iter_mut() {
                for vertex in shape.vertices.iter_mut() {
                    let tmp = Vec4::new(vertex.0.x, vertex.0.y, vertex.0.z, 1.0);
                    let new_vertex = transformation_matrix * tmp;
                    vertex.0 = new_vertex.xyz();
                }
            }

            shapes
        }
    }

    /// DRAWING FLAT CIRCLE LINE
    pub struct Circle {
        center: Vec3,
        radius: f32,
        number_of_segments: u32,
        color: Vec3,
    }

    impl DrawableShape for Circle {
        fn generate_shape(&self, offset: u32) -> Vec<Shape> {
            let mut circle = Shape {
                vertices: Vec::with_capacity(self.number_of_segments as usize),
                draw_call: (
                    gl::LINE_STRIP as u32,
                    offset,
                    self.number_of_segments as u32,
                ),
            };
            for i in 0..self.number_of_segments {
                let t = 2.0 * PI * i as f32 / (self.number_of_segments as f32 - 1.0);
                let current_point = self.radius * Vec3::new(t.cos(), t.sin(), 0.0);
                circle
                    .vertices
                    .push((current_point, self.color, Vec2::zeros()));
            }
            vec![circle]
        }

        // fn generate_transformed_shape(
        //     &self,
        //     offset: u32,
        //     transformation_matrix: &Mat4,
        // ) -> Vec<Shape> {
        //     let mut shapes = self.generate_shape(offset);

        //     for shape in shapes.iter_mut() {
        //         for vertex in shape.vertices.iter_mut() {
        //             let tmp = Vec4::new(vertex.0.x, vertex.0.y, vertex.0.z, 1.0);
        //             let new_vertex = transformation_matrix * tmp;
        //             vertex.0 = new_vertex.xyz();
        //         }
        //     }

        //     shapes
        // }
    }

    /// DRAWING A QUAD
    pub struct Quad {
        center: Vec3,
        width: f32,
        height: f32,
        color: Vec3,
    }

    impl Quad {
        pub fn new_from_two_points(p0: &Vec2, p1: &Vec2, width: f32, color: &Vec3) -> Self {
            let height = nalgebra_glm::length(&(p1 - p0));
            let center = 0.5 * (p0 + p1);

            Self {
                center: Vec3::new(center.x, center.y, 0.0),
                width: width,
                height: height,
                color: *color,
            }
        }
    }
    impl DrawableShape for Quad {
        fn generate_shape(&self, offset: u32) -> Vec<Shape> {
            let mut vertices = [
                (
                    self.center + Vec3::new(-self.width / 2.0, -self.height / 2.0, 0.0),
                    self.color,
                    Vec2::new(0.0, 0.0),
                ),
                (
                    self.center + Vec3::new(self.width / 2.0, self.height / 2.0, 0.0),
                    self.color,
                    Vec2::new(1.0, 1.0),
                ),
                (
                    self.center + Vec3::new(-self.width / 2.0, self.height / 2.0, 0.0),
                    self.color,
                    Vec2::new(0.0, 1.0),
                ),
                ////////////////////////////////////////////////////////////////
                (
                    self.center + Vec3::new(-self.width / 2.0, -self.height / 2.0, 0.0),
                    self.color,
                    Vec2::new(0.0, 0.0),
                ),
                (
                    self.center + Vec3::new(self.width / 2.0, -self.height / 2.0, 0.0),
                    self.color,
                    Vec2::new(1.0, 0.0),
                ),
                (
                    self.center + Vec3::new(self.width / 2.0, self.height / 2.0, 0.0),
                    self.color,
                    Vec2::new(1.0, 1.0),
                ),
            ]
            .to_vec();

            vec![Shape {
                vertices: vertices,
                draw_call: (gl::TRIANGLES as _, offset, 6),
            }]
        }
    }

    /// TRIANGLE

    struct Triangle {
        start_point: Vec2,
        end_point: Vec2,
        width: f32,
        color: Vec3,
    }

    impl DrawableShape for Triangle {
        fn generate_shape(&self, offset: u32) -> Vec<Shape> {
            let dir = nalgebra_glm::normalize(&(self.end_point - self.start_point));

            let left = Vec2::new(-dir.y, dir.x);
            let vertices = [
                self.start_point + self.width * left,
                self.start_point - self.width * left,
                self.end_point,
            ];

            let vertices = vertices.map(|v| (Vec3::new(v.x, v.y, 0.0), self.color, Vec2::zeros()));

            vec![Shape {
                vertices: vertices.to_vec(),
                draw_call: (gl::TRIANGLES as _, offset, 3),
            }]
        }
    }
}

fn main() {
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
