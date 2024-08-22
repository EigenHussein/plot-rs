// pub struct FullVertex {
//     position: Vec3,
//     normal: Vec3,
//     color: Vec3,
//     txcoord: Vec2,
// }

pub mod TwoDim {

    use nalgebra_glm::{Vec2, Vec3};
    pub struct Quad; /* {
                         vertices: [Vec3;6]
                     } */
    impl Quad {
        pub fn new_from_2points(
            p0: &Vec3,
            p1: &Vec3,
            width: f32,
            normal: Option<&Vec3>,
        ) -> [Vec3; 6] {
            let normal_dir = normal.unwrap_or(&Vec3::z()).normalize();
            let U = (p1 - p0).normalize();
            let V = U.cross(&normal_dir);

            // vertices

            [
                p1 + 0.5 * width * V,
                p1 - 0.5 * width * V,
                p0 - 0.5 * width * V,
                p0 - 0.5 * width * V,
                p0 + 0.5 * width * V,
                p1 + 0.5 * width * V,
            ]
        }
    }

    pub struct Triangle;

    impl Triangle {
        pub fn new_triangle(
            start_point: &Vec3,
            normal: Option<&Vec3>,
            size: (f32, f32),
        ) -> [Vec3; 3] {
            let normal_dir = normal.unwrap_or(&Vec3::z()).normalize();
            let end_point = start_point * (1.0 + size.1 / nalgebra_glm::length(start_point));
            let U = (end_point - start_point).normalize();
            let V = U.cross(&normal_dir);

            [
                start_point + size.0 * 0.5 * V,
                start_point - size.0 * 0.5 * V,
                end_point,
            ]
        }
    }
}

pub mod Shapes {

    pub type ShapeVertexType = (Vec4, Vec4, Vec2);
    use std::f32::consts::PI;

    use gl::types;
    use nalgebra_glm::{Mat4, Vec2, Vec3, Vec4};

    use crate::{
        buffers::{ArrayBuffer, GLAttrib},
        camera::Camera,
        shader_builder::{shader::CompileProgram, RawShader},
    };
    #[derive(Clone, Copy, Debug)]
    pub struct ShapeDrawCall {
        pub(crate) mode: types::GLenum,
        pub(crate) first: types::GLint,
        pub(crate) count: types::GLsizei,
    }
    impl ShapeDrawCall {
        pub fn draw(&self) {
            unsafe {
                gl::DrawArrays(self.mode, self.first, self.count);
            }
        }
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
        pub vertices: Vec<ShapeVertexType>,
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
                    draw_call.draw();
                }
            }
        }
    }

    pub trait DrawableShape {
        ///
        /// ### Description
        /// Generate vertices + draw call info
        fn generate_shape(&self, offset: u32) -> Shape;
        // fn get_vertices_mut(&mut self) -> &Vec<VertexType>;
        fn generate_transformed_shape(&self, offset: u32, transformation_matrix: &Mat4) -> Shape {
            let mut shape = self.generate_shape(offset);

            for vertex in shape.vertices.iter_mut() {
                let tmp = Vec4::new(vertex.0.x, vertex.0.y, vertex.0.z, 1.0);
                let new_vertex = transformation_matrix * tmp;
                vertex.0 = new_vertex;
            }

            shape
        }
    }

    /// DRAWING FLAT CIRCLE LINE
    pub struct Circle {
        center: Vec3,
        radius: f32,
        number_of_segments: u32,
        color: Vec4,
    }

    impl Circle {
        pub fn new_circle(
            center: Vec3,
            radius: f32,
            color: Option<Vec4>,
            number_of_segments: Option<u32>,
        ) -> Self {
            let tmp = Self {
                center: center,
                radius: radius,
                color: color.unwrap_or(Vec4::new(1.0, 1.0, 1.0, 1.0)),
                number_of_segments: number_of_segments.unwrap_or(100),
            };
            return tmp;
        }
    }
    impl DrawableShape for Circle {
        fn generate_shape(&self, offset: u32) -> Shape {
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
                circle.vertices.push((
                    Vec4::new(current_point.x, current_point.y, current_point.z, 1.0),
                    self.color,
                    Vec2::zeros(),
                ));
            }
            circle
        }
    }

    #[derive(Debug, Default)]
    /// DRAWING A QUAD
    pub struct Quad {
        p1: Vec3,
        p2: Vec3,
        width: f32,
        color: Vec4,
    }

    impl Quad {
        pub fn new_from_two_points(
            p1: &Vec2,
            p2: &Vec2,
            width: f32,
            color: &Vec4,
            z_val: f32,
        ) -> Self {
            Self {
                p1: Vec3::new(p1.x, p1.y, z_val),
                p2: Vec3::new(p2.x, p2.y, z_val),
                width: width,
                color: *color,
            }
        }
    }
    impl DrawableShape for Quad {
        fn generate_shape(&self, offset: u32) -> Shape {
            let dir = nalgebra_glm::normalize(&(self.p2 - self.p1));
            let norm = Vec3::new(-dir.y, dir.x, 0.0);

            let mut vertices = [
                (
                    //self.center + Vec3::new(-self.width / 2.0, -self.height / 2.0, 0.0),
                    self.p1 - 0.5 * self.width * norm,
                    self.color,
                    Vec2::new(0.0, 0.0),
                ),
                (
                    self.p2 + 0.5 * self.width * norm,
                    self.color,
                    Vec2::new(1.0, 1.0),
                ),
                (
                    self.p2 - 0.5 * self.width * norm,
                    self.color,
                    Vec2::new(0.0, 1.0),
                ),
                ////////////////////////////////////////////////////////////////
                (
                    self.p1 - 0.5 * self.width * norm,
                    self.color,
                    Vec2::new(0.0, 0.0),
                ),
                (
                    self.p1 + 0.5 * self.width * norm,
                    self.color,
                    Vec2::new(1.0, 0.0),
                ),
                (
                    self.p2 + 0.5 * self.width * norm,
                    self.color,
                    Vec2::new(1.0, 1.0),
                ),
            ]
            .to_vec();

            Shape {
                vertices: vertices
                    .iter()
                    .map(|v| (Vec4::new(v.0.x, v.0.y, v.0.z, 1.0), v.1, v.2))
                    .collect(),
                draw_calls: vec![ShapeDrawCall::new_triangles(offset as _, 2)], //(gl::TRIANGLES as _, offset, 6),
            }
        }
    }

    /// TRIANGLE

    pub struct Triangle {
        start_point: Vec2,
        end_point: Vec2,
        level: f32,
        width: f32,
        color: Vec4,
    }

    impl Triangle {
        pub fn new(p1: Vec2, p2: Vec2, width: f32, color: Option<Vec4>, level: f32) -> Self {
            Self {
                start_point: p1,
                end_point: p2,
                level: level,
                width: width,
                color: color.unwrap_or(Vec4::new(1.0, 0.6, 0.2, 1.0)),
            }
        }
    }
    impl DrawableShape for Triangle {
        fn generate_shape(&self, offset: u32) -> Shape {
            let dir = nalgebra_glm::normalize(&(self.end_point - self.start_point));

            let left = Vec2::new(-dir.y, dir.x);
            let vertices = [
                self.start_point + self.width * left,
                self.start_point - self.width * left,
                self.end_point,
            ];

            let vertices = vertices.map(|v| {
                (
                    Vec4::new(v.x, v.y, self.level, 1.0),
                    self.color,
                    Vec2::zeros(),
                )
            });

            Shape {
                vertices: vertices.to_vec(),
                draw_calls: vec![ShapeDrawCall::new_triangles(offset as _, 1)],
            }
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

        let mut vertices: Vec<ShapeVertexType> = Vec::with_capacity(vertex_counter);
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
        let mut combined_shapes = combine_triangles_level0(shapes);

        // sort draw_calls

        combined_shapes
            .draw_calls
            .sort_by(|a, b| a.mode.cmp(&b.mode));

        let mut sorted_vertices: Vec<ShapeVertexType> =
            Vec::with_capacity(combined_shapes.vertices.len());
        let mut draw_calls: Vec<ShapeDrawCall> = Vec::new();

        draw_calls.push(combined_shapes.draw_calls[0]);
        draw_calls.last_mut().unwrap().first = 0;
        sorted_vertices.extend_from_slice(
            &combined_shapes.vertices.as_slice()[(combined_shapes.draw_calls[0].first) as usize
                ..(combined_shapes.draw_calls[0].first + combined_shapes.draw_calls[0].count)
                    as usize],
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

    pub struct DynamicLineFunction {
        buffer: ArrayBuffer,
        program: RawShader,
        steps: u32,
    }

    impl DynamicLineFunction {
        pub fn bind_camera(&mut self, cam: &mut Camera) -> &mut Self {
            cam.add_program(self.program.get_id());
            cam.update_uniforms(None);
            self
        }
        pub fn draw_on(&self, line_width: f32) {
            unsafe {
                gl::LineWidth(line_width);
                gl::UseProgram(self.program.get_id());
                self.buffer.vao.bind();
                self.buffer.vbo.bind();
                gl::DrawArrays(gl::LINE_STRIP, 0, self.steps as _);
            }
        }
        fn default_program() -> RawShader {
            RawShader::wrap(CompileProgram(
                "shaders/shapes/vertex.glsl",
                "shaders/shapes/fragment.glsl",
                None,
            ))
        }
        pub fn generate_shape<LineFunc, ColorFunc>(
            line_function: LineFunc,
            t0: f32,
            t1: f32,
            steps: u32,
            color_func: ColorFunc,
        ) -> Shape
        where
            LineFunc: Fn(f32) -> Vec4,
            ColorFunc: Fn(f32) -> Vec4,
        {
            let mut vertices = Vec::<ShapeVertexType>::with_capacity(steps as usize);
            for i in 0..steps {
                let t = t0 + (t1 - t0) * i as f32 / (steps as f32 - 1.0);
                vertices.push((line_function(t), color_func(t), Vec2::zeros()));
            }
            Shape {
                vertices: vertices,
                draw_calls: [ShapeDrawCall::new_line_strip(0, steps as i32)].to_vec(),
            }
        }
        // fn generate_vertices
        pub fn new<LineFunc, ColorFunc>(
            line_function: LineFunc,
            t0: f32,
            t1: f32,
            steps: u32,
            color_func: ColorFunc,
        ) -> Self
        where
            LineFunc: Fn(f32) -> Vec4,
            ColorFunc: Fn(f32) -> Vec4,
        {
            // let mut vertices = Vec::<ShapeVertexType>::with_capacity(steps as usize);
            // for i in 0..steps {
            //     let t = t0 + (t1 - t0) * i as f32 / (steps as f32);
            //     vertices.push((line_function(t), color_func(t), Vec2::zeros()));
            // }

            let line_shape = Self::generate_shape(line_function, t0, t1, steps, color_func);

            let mut buffer = ArrayBuffer::default_buffer_from_slice(
                &line_shape.vertices,
                None,
                &[
                    GLAttrib::new(0, 4, 10, 0),
                    GLAttrib::new(1, 4, 10, 4),
                    GLAttrib::new(2, 2, 10, 8),
                ]
                .to_vec(),
                true,
            );
            Self {
                buffer: buffer,
                program: Self::default_program(),
                steps: steps,
            }
        }

        pub fn update_line_inplace<LineFunc>(
            &mut self,
            line_function: LineFunc,
            t0: f32,
            t1: f32,
        ) -> &mut Self
        where
            LineFunc: Fn(f32) -> Vec4,
        {
            unsafe {
                let buf = std::slice::from_raw_parts_mut::<ShapeVertexType>(
                    gl::MapNamedBuffer(self.buffer.vbo.unwrap_buffer(), gl::WRITE_ONLY) as _,
                    self.steps as _,
                );
                for i in 0..self.steps as usize {
                    let t = t0 + (t1 - t0) * i as f32 / (self.steps as f32 - 1.0);
                    buf[i].0 = line_function(t);
                }
                gl::UnmapNamedBuffer(self.buffer.vbo.unwrap_buffer());
            }

            self
        }

        // pub fn update_function<LineFunc>(
        //     &mut self,
        //     line_function: LineFunc,
        //     t0: f32,
        //     t1: f32,
        // ) -> &mut Self
        // where
        //     LineFunc: Fn(f32) -> Vec4,
        // {
        //     let b
        //     self
        // }
    }
}

pub mod Surfaces {
    use nalgebra_glm::{Vec2, Vec4};

    use crate::{
        buffers::{ArrayBuffer, GLAttrib},
        camera::Camera,
        shader_builder::{shader::CompileProgram, RawShader},
    };

    use super::Shapes::{Shape, ShapeDrawCall, ShapeVertexType};

    pub struct ParametricSurface {
        program: RawShader,
        buffer: ArrayBuffer,
        steps: (u32, u32),
        number_of_indices: u32,
    }
    impl ParametricSurface {
        pub fn bind_camera(&mut self, cam: &mut Camera) -> &mut Self {
            cam.add_program(self.program.get_id());
            cam.update_uniforms(None);
            self
        }
        pub fn draw_on(&self) {
            unsafe {
                // gl::LineWidth(line_width);
                gl::UseProgram(self.program.get_id());
                self.buffer.vao.bind();
                self.buffer.vbo.bind();
                self.buffer.ebo.bind();
                //gl::DrawArrays(gl::TRIANGLES, 0, self.number_of_vertices as _);
                gl::DrawElements(
                    gl::TRIANGLES,
                    self.number_of_indices as _,
                    gl::UNSIGNED_INT,
                    0 as _,
                );
            }
        }
        fn default_program() -> RawShader {
            RawShader::wrap(CompileProgram(
                "shaders/shapes/vertex.glsl",
                "shaders/shapes/fragment.glsl",
                None,
            ))
        }
        // fn generate_grid_indices(width: usize, height: usize) -> Vec<u32> {
        //     let mut indices = Vec::new();

        //     for y in 0..(height - 1) {
        //         for x in 0..(width - 1) {
        //             let top_left = (y * width + x) as u32;
        //             let top_right = (y * width + x + 1) as u32;
        //             let bottom_left = ((y + 1) * width + x) as u32;
        //             let bottom_right = ((y + 1) * width + x + 1) as u32;

        //             // First triangle
        //             indices.push(top_left);
        //             indices.push(bottom_left);
        //             indices.push(top_right);

        //             // Second triangle
        //             indices.push(top_right);
        //             indices.push(bottom_left);
        //             indices.push(bottom_right);
        //         }
        //     }

        //     indices
        // }
        pub fn generate_surface_indices(usteps: u32, vsteps: u32) -> Vec<u32> {
            let number_of_indices = ((usteps) * (vsteps)) * 6;
            let mut indices: Vec<u32> = Vec::with_capacity(number_of_indices as usize);

            for v in 0..vsteps {
                for u in 0..usteps {
                    let current_index = v * (usteps + 1) + u;

                    //Triangle 1
                    indices.push(current_index);
                    indices.push(current_index + 1 + usteps + 1);
                    indices.push(current_index + 1);

                    //Triangle 2
                    indices.push(current_index);
                    indices.push(current_index + usteps + 1);
                    indices.push(current_index + 1 + usteps + 1);
                }
            }

            indices
        }
        // pub fn generate_shape<LineFunc, ColorFunc>(
        //     line_function: LineFunc,
        //     u: (f32, f32),
        //     v: (f32, f32),
        //     steps: (u32, u32),
        //     color_func: ColorFunc,
        // ) -> Shape
        // where
        //     LineFunc: Fn(f32, f32) -> Vec4,
        //     ColorFunc: Fn(f32, f32) -> Vec4,
        // {
        //     let mut vertices = Vec::<ShapeVertexType>::with_capacity(steps as usize);
        //     // for i in 0..steps {
        //     //     let t = t0 + (t1 - t0) * i as f32 / (steps as f32 - 1.0);
        //     //     vertices.push((line_function(t), color_func(t), Vec2::zeros()));
        //     // }
        //     Shape {
        //         vertices: vertices,
        //         draw_calls: [ShapeDrawCall::new_line_strip(0, steps as i32)].to_vec(),
        //     }
        // }
        // fn generate_vertices
        pub fn new<SurfFunc, ColorFunc>(
            surface_function: SurfFunc,
            u: (f32, f32),
            v: (f32, f32),
            steps: (u32, u32),
            color_func: ColorFunc,
        ) -> Self
        where
            SurfFunc: Fn(f32, f32) -> Vec4,
            ColorFunc: Fn(f32, f32) -> Vec4,
        {
            //let line_shape = Self::generate_shape(line_function, t0, t1, steps, color_func);

            let du = (u.1 - u.0) / ((steps.0) as f32);
            let dv = (v.1 - v.0) / ((steps.1) as f32);

            let mut vertices: Vec<ShapeVertexType> =
                Vec::with_capacity(((steps.0 + 1) * (steps.1 + 1) * 6) as usize);

            for v_idx in 0..steps.1 + 1 {
                let v = v.0 + dv * v_idx as f32;
                for u_idx in 0..steps.0 + 1 {
                    let u = u.0 + du * u_idx as f32;
                    vertices.push((surface_function(u, v), color_func(u, v), Vec2::zeros()));
                }
            }

            let indices = Self::generate_surface_indices(steps.0, steps.1);

            let mut buffer = ArrayBuffer::default_buffer_from_slice(
                &vertices,
                Some(indices.as_slice()),
                &[
                    GLAttrib::new(0, 4, 10, 0),
                    GLAttrib::new(1, 4, 10, 4),
                    GLAttrib::new(2, 2, 10, 8),
                ]
                .to_vec(),
                true,
            );
            Self {
                buffer: buffer,
                program: Self::default_program(),
                steps: steps,
                number_of_indices: indices.len() as _,
            }
        }

        pub fn update_surface_inplace<SurfFunc>(
            &mut self,
            surface_function: SurfFunc,
            u: (f32, f32),
            v: (f32, f32),
        ) -> &mut Self
        where
            SurfFunc: Fn(f32, f32) -> Vec4,
        {
            unsafe {
                let buf = std::slice::from_raw_parts_mut::<ShapeVertexType>(
                    gl::MapNamedBuffer(self.buffer.vbo.unwrap_buffer(), gl::WRITE_ONLY) as _,
                    self.number_of_indices as _,
                );

                let du = (u.1 - u.0) / ((self.steps.0) as f32);
                let dv = (v.1 - v.0) / ((self.steps.1) as f32);

                // let mut vertices: Vec<ShapeVertexType> =
                //     Vec::with_capacity(((steps.0 + 1) * (steps.1 + 1) * 6) as usize);

                for v_idx in 0..self.steps.1 + 1 {
                    let v = v.0 + dv * v_idx as f32;
                    for u_idx in 0..self.steps.0 + 1 {
                        let u = u.0 + du * u_idx as f32;
                        // vertices.push((surface_function(u, v), color_func(u, v), Vec2::zeros()));
                        let vertex_idx = (u_idx + v_idx * (self.steps.0 + 1)) as usize;
                        buf[vertex_idx].0 = surface_function(u, v);
                        // buf[vertex_idx] = (surface_function(u, v), color_func(u, v), Vec2::zeros());
                    }
                }

                gl::UnmapNamedBuffer(self.buffer.vbo.unwrap_buffer());
            }

            self
        }
    }
}
