use glfw::Context;

use nalgebra_glm::{Vec2, Vec3, Vec4};
use plot_rs::{
    buffers::{ArrayBuffer, GLAttrib},
    camera::Camera,
    shader_builder::shader::CompileProgram,
    shapes::TwoDim,
    window::Window,
};
use Shapes::{combine_triangles_level1, Circle, DrawableShape};
// TODO Add more shapes
pub mod Shapes {

    type VertexType = (Vec3, Vec3, Vec2);
    use std::f32::consts::PI;

    use gl::types;
    use nalgebra_glm::{Mat4, Vec2, Vec3, Vec4};
    #[derive(Clone, Copy, Debug)]
    pub struct ShapeDrawCall {
        pub(crate) mode: types::GLenum,
        pub(crate) first: types::GLint,
        pub(crate) count: types::GLsizei,
    }
    impl ShapeDrawCall {
        pub fn new_triangles(offset: i32, number_of_triangles: i32) -> Self {
            Self {
                mode: gl::TRIANGLES,
                first: offset,
                count: 3 * number_of_triangles,
            }
        }
        pub fn new_line_strip(offset: i32, number_of_vertices: i32) -> Self {
            Self {
                mode: gl::LINE_STRIP,
                first: offset,
                count: number_of_vertices,
            }
        }
        pub fn new_lines(offset: i32, number_of_vertices: i32) -> Self {
            Self {
                mode: gl::LINES,
                first: offset,
                count: number_of_vertices,
            }
        }
        pub fn new_custom(mode: u32, offset: i32, number_of_vertices: i32) -> Self {
            Self {
                mode: mode,
                first: offset,
                count: number_of_vertices,
            }
        }

        pub fn is_combinable(&self) -> bool {
            match self.mode {
                gl::TRIANGLES => true,
                gl::LINES => true,
                _ => false,
            }
        }
    }

    #[derive(Default, Clone, Debug)]
    pub struct Shape {
        /// ### Vertex | Color | TXCoord
        pub vertices: Vec<VertexType>,
        /// ### mode | offset | count
        pub draw_calls: Vec<ShapeDrawCall>,
    }

    impl Shape {
        pub fn draw(&self) {
            unsafe {
                // gl::DrawArrays(
                //     self.draw_call.0 as _,
                //     self.draw_call.1 as _,
                //     self.draw_call.2 as _,
                // );
                for draw_call in self.draw_calls.iter() {
                    gl::DrawArrays(draw_call.mode, draw_call.first, draw_call.count)
                }
            }
        }
    }

    pub trait DrawableShape {
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

    impl Circle {
        pub fn new_circle(
            center: Vec3,
            radius: f32,
            color: Option<Vec3>,
            number_of_segments: Option<u32>,
        ) -> Self {
            let tmp = Self {
                center: center,
                radius: radius,
                color: color.unwrap_or(Vec3::new(1.0, 1.0, 1.0)),
                number_of_segments: number_of_segments.unwrap_or(100),
            };
            return tmp;
        }
    }
    impl DrawableShape for Circle {
        fn generate_shape(&self, offset: u32) -> Vec<Shape> {
            let mut circle = Shape {
                vertices: Vec::with_capacity(self.number_of_segments as usize),
                draw_calls: vec![ShapeDrawCall::new_line_strip(
                    offset as _,
                    self.number_of_segments as _,
                )],
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
                draw_calls: vec![ShapeDrawCall::new_triangles(offset as _, 2)], //(gl::TRIANGLES as _, offset, 6),
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
                draw_calls: vec![ShapeDrawCall::new_triangles(offset as _, 1)],
            }]
        }
    }

    pub fn combine_triangles_level0(shapes: Vec<Shape>) -> Shape {
        let mut vertex_counter = 0;
        let mut call_counter = 0;
        /* for shape_vec in shapes  */
        {
            for shape in shapes.iter() {
                vertex_counter += shape.vertices.len();
                call_counter += shape.draw_calls.len();
            }
        }

        let mut vertices: Vec<VertexType> = Vec::with_capacity(vertex_counter);
        let mut draw_calls: Vec<ShapeDrawCall> = Vec::with_capacity(call_counter);
        /*  for shape_vec in shapes  */
        {
            for shape in shapes.iter() {
                vertices.extend(&shape.vertices);
                draw_calls.extend(&shape.draw_calls);
                // vertex_counter += shape.vertices.len();
                // call_counter += shape.draw_calls.len();
            }
        }
        for i in 1..draw_calls.len() {
            draw_calls[i].first = draw_calls[i - 1].first + draw_calls[i - 1].count;
        }
        Shape {
            vertices: vertices,
            draw_calls: draw_calls,
        }
    }

    //TODO add sorting
    pub fn combine_triangles_level1(shapes: Vec<Shape>) -> Shape {
        //find the number of vertices
        // let mut vertex_counter = 0;
        // let mut call_counter = 0;

        // for shape in shapes {
        //     vertex_counter += shape.vertices.len();
        //     call_counter += shape.draw_calls.len();
        // }

        // let mut output_shape: Vec<VertexType> = Vec::with_capacity(vertex_counter);
        // let mut draw_calls: Vec<ShapeDrawCall> = Vec::with_capacity(call_counter);

        // // fix draw_calls

        // for i in 1..draw_calls.len() {
        //     draw_calls[i].first = draw_calls[i - 1].first + draw_calls[i - 1].count;
        // }
        // draw_calls.sort_by(|a, b| a.mode.cmp(&b.mode));

        // for shape in shapes {
        //     output_shape.extend(shape.vertices);
        //     draw_calls.extend(shape.draw_calls);
        // }

        let mut combined_shapes = combine_triangles_level0(shapes);

        // sort draw_calls

        combined_shapes
            .draw_calls
            .sort_by(|a, b| a.mode.cmp(&b.mode));

        let mut sorted_vertices: Vec<VertexType> =
            Vec::with_capacity(combined_shapes.vertices.len());
        let mut draw_calls: Vec<ShapeDrawCall> = Vec::new();

        draw_calls.push(combined_shapes.draw_calls[0]);
        draw_calls.last_mut().unwrap().first = 0;
        sorted_vertices.extend_from_slice(
            &combined_shapes.vertices.as_slice()[0..draw_calls[0].count as usize],
        );

        let mut call_walker = 1;
        while call_walker < combined_shapes.draw_calls.len() {
            // get the previous and current call
            let previous_call = draw_calls.last_mut().unwrap();
            let current_call = &combined_shapes.draw_calls[call_walker];

            // if the two calls are the same and both are combinable
            // add the vertices
            // extend the previous call by increasing its count
            if current_call.mode == previous_call.mode && current_call.is_combinable() {
                sorted_vertices.extend_from_slice(
                    &combined_shapes.vertices.as_slice()[current_call.first as usize
                        ..(current_call.first + current_call.count) as usize],
                );
                previous_call.count += current_call.count;

            // if not create a new call
            } else {
                let mut new_call_type = *current_call;
                new_call_type.first = previous_call.first + previous_call.count;
                draw_calls.push(new_call_type);
                // draw_calls.push(*current_call);
                //sorted_vertices.push(combined_shapes.vertices[call_walker]);
                sorted_vertices.extend_from_slice(
                    &combined_shapes.vertices.as_slice()[current_call.first as usize
                        ..(current_call.first + current_call.count) as usize],
                );
            }
            call_walker += 1;
        }

        Shape {
            vertices: sorted_vertices,
            draw_calls: draw_calls,
        }
    }
}

fn main() {
    let quad1 = &Shapes::Quad::new_from_two_points(
        &Vec2::new(0.0, 0.0),
        &Vec2::new(0.0, 1.0),
        0.5,
        &Vec3::new(1.0, 0.5, 0.3),
    )
    .generate_shape(0)[0];
    let circle = &Shapes::Circle::new_circle(Vec3::zeros(), 1.0, None, None).generate_shape(0)[0];
    let quad2 = &Shapes::Quad::new_from_two_points(
        &Vec2::new(0.0, 0.0),
        &Vec2::new(1.0, 1.0),
        0.5,
        &Vec3::new(1.0, 0.5, 0.3),
    )
    .generate_shape(0)[0];
    let circle2 = &Shapes::Circle::new_circle(Vec3::zeros(), 1.0, None, Some(20)).generate_shape(0)[0];

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

    let circle_info = &circle.generate_shape(0)[0];

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
