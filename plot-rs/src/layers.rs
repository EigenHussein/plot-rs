trait StaticLayer {
    fn get_program_id(&self) -> u32;
    fn draw_on(&self) -> u32;
}
pub trait Drawable {
    fn draw_on(&self);
}

// pub struct Layer {
//     /*
//        shader : ShaderProgram,
//        buffer : ArrayBuffer,
//        drawcall:
//     */
// }

pub mod MathLayers {
    use nalgebra_glm::{Mat4, Vec3, Vec4};

    use crate::{
        buffers::{ArrayBuffer, GLAttrib},
        scene::SceneLayer,
        shader_builder::{RawShader, ShaderAttribute, ShaderAttributeType, ShaderBuilder},
    };

    use super::Drawable;

    /// ### Description
    /// * A structure for orthogonal coordinates, it also can map vectors using specific coordinates
    #[derive(Clone, Copy)]
    pub struct OrthogonalCoordinates {
        pub model: Mat4,
    }

    impl OrthogonalCoordinates {
        /// ### Description
        /// * create new coordinates, by giving it two axes the third is calculated using cross product
        pub fn new(origin: Vec3, U: Vec3, V: Vec3) -> Self {
            let W = nalgebra_glm::cross(&U, &V);
            let model = Mat4::new(
                U.x, V.x, W.x, origin.x, U.y, V.y, W.y, origin.y, U.z, V.z, W.z, origin.z, 0.0,
                0.0, 0.0, 1.0,
            );
            Self { model: model }
        }

        pub fn new_from_normal(origin: Vec3, normal: Vec3, up: Vec3) -> Self {
            let right = nalgebra_glm::cross(&up, &normal);
            Self::new(origin, right, up)
        }
        /// ### Description
        /// * Calculate the vector for given bases `vec`
        pub fn map(&self, vec: &Vec3) -> Vec3 {
            (self.model * Vec4::new(vec.x, vec.y, vec.z, 1.0)).xyz()
        }

        /// ### Description
        /// * Calculate the vector for given bases `vec`
        pub fn map_4f(&self, vec: &Vec4) -> Vec4 {
            self.model * vec
        }
        /// ### Description
        /// * Generates a new coordinate with translated origin
        pub fn translate_coordinates(&self, transition_vector: Vec3) -> Self {
            let mut new_coords = self.model;
            new_coords[12] += transition_vector.x;
            new_coords[13] += transition_vector.y;
            new_coords[14] += transition_vector.z;
            Self { model: new_coords }
        }
        pub fn rotate_coordinates(&self, rotation_matrix: Mat4) -> Self {
            Self {
                model: rotation_matrix * self.model,
            }
        }
    }
    impl Default for OrthogonalCoordinates {
        fn default() -> Self {
            Self {
                model: Mat4::identity(),
            }
        }
    }

    pub struct LineLayer {
        // pub shader_builder: ShaderBuilder,
        pub shader: RawShader,
        pub buffer: ArrayBuffer,
        number_of_vertices: i32,
        pub vertices: Option<Vec<(Vec3, Vec3)>>,
    }

    // impl Default for LineLayer {
    //     fn default() -> Self {
    //         let mut builder = ShaderBuilder::default();
    //         // builder.
    //         // LineLayer {
    //         //     shader_builder: ShaderBuilder::default(),
    //         //     shader: None,
    //         //     buffer: ArrayBuffer::uninitialized(),
    //         // }
    //     }
    // }
    impl LineLayer {
        pub fn simple_colored_line<LineFunc, ColorFunc>(
            line_function: LineFunc,
            t0: f32,
            t1: f32,
            steps: u32,
            coordinates: Option<OrthogonalCoordinates>,
            color_function: ColorFunc,
            static_line: bool,
        ) -> Self
        where
            LineFunc: Fn(f32) -> Vec3,
            ColorFunc: Fn(f32) -> Vec3,
            // T: Sized,
        {
            let shader = ShaderBuilder::default()
                .set_vertex_shader_input_attributes(Some(
                    [
                        ShaderAttribute::new_input_attribute(
                            Some(0),
                            "position".to_string(),
                            crate::shader_builder::ShaderAttributeType::Vec3,
                        ),
                        ShaderAttribute::new_input_attribute(
                            Some(1),
                            "color".to_string(),
                            ShaderAttributeType::Vec3,
                        ),
                    ]
                    .to_vec(),
                ))
                .set_color_assignment(crate::shader_builder::ColorAssignment::PerVertex)
                .add_vertex2fragment_link(
                    ShaderAttribute::new_output_attribute(
                        None,
                        "v_col".to_string(),
                        ShaderAttributeType::Vec3,
                    ),
                    "color".to_string(),
                )
                .set_vertex_transformation(
                    crate::shader_builder::VertexTransformation::ProjectionView,
                )
                .compile_shader();

            let vertices: Vec<(Vec3, Vec3)> = if let Some(coords) = coordinates {
                // Generate vertices for default coordinates
                (0..steps)
                    .into_iter()
                    .map(|i| {
                        let t = t0 + (t1 - t0) * i as f32 / (steps as f32 - 1.0);
                        let tmp = line_function(t);
                        (
                            (coords.model * Vec4::new(tmp.x, tmp.y, tmp.z, 1.0)).xyz(),
                            color_function(t),
                        )
                    })
                    .collect()
            } else {
                // Generate vertices for specific coordinates
                (0..steps)
                    .into_iter()
                    .map(|i| {
                        let t_color: f32 = i as f32 / (steps as f32 - 1.0);
                        let t_func = t0 + (t1 - t0) * i as f32 / steps as f32;
                        (line_function(t_func), color_function(t_color))
                    })
                    .collect()
            };

            let mut buffer = ArrayBuffer::default_buffer_from_slice(
                vertices.as_slice(),
                None,
                &[GLAttrib::new(0, 3, 6, 0), GLAttrib::new(1, 3, 6, 3)].to_vec(),
                static_line,
            );
            Self {
                number_of_vertices: steps as _,
                shader: shader,
                buffer: buffer,
                vertices: if static_line == false {
                    Some(vertices)
                } else {
                    None
                },
            }
        }
        pub fn update_line_function<LineFunc>(
            &mut self,
            funciton: LineFunc,
            t0: f32,
            t1: f32,
        ) -> &mut Self
        where
            LineFunc: Fn(f32) -> Vec3,
        {
            if let Some(ref mut vertices) = self.vertices {
                let n_vertices = self.number_of_vertices as usize;
                for i in 0..n_vertices {
                    let t = t0 + (t1 - t0) * i as f32 / (n_vertices as f32 - 1.0);
                    vertices[i].0 = funciton(t);
                }
                self.buffer.vbo.update_buffer_slice(0, vertices.as_slice());
            }

            self
        }
        pub fn update_line_color<ColorFunc>(&mut self, color_func: ColorFunc) -> &mut Self
        where
            ColorFunc: Fn(f32) -> Vec3,
        {
            if let Some(ref mut vertices) = self.vertices {
                let n_vertices = self.number_of_vertices as usize;
                for i in 0..n_vertices {
                    let t = i as f32 / (n_vertices as f32 - 1.0);
                    vertices[i].1 = color_func(t);
                }
                self.buffer.vbo.update_buffer_slice(0, vertices.as_slice());
            }

            self
        }
    }

    impl Drawable for LineLayer {
        fn draw_on(&self) {
            self.shader.use_shader();
            self.buffer.vao.bind();
            unsafe {
                gl::DrawArrays(gl::LINE_STRIP, 0, self.number_of_vertices);
            }
        }
    }
    impl SceneLayer for LineLayer {
        fn bind_camera(&self, camera: &mut crate::camera::Camera) {
            camera.add_program(self.get_program_id());
        }
        fn draw(&self) {
            self.shader.use_shader();
            self.buffer.vao.bind();
            unsafe {
                gl::DrawArrays(gl::LINE_STRIP, 0, self.number_of_vertices);
            }
        }
        fn get_program_id(&self) -> u32 {
            self.shader.get_id()
        }
    }
    pub struct LineLayerConfiguration {}
}
