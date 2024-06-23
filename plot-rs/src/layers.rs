trait StaticLayer {
    fn get_program_id(&self) -> u32;
    fn draw_on(&self) -> u32;
}

// pub struct Layer {
//     /*
//        shader : ShaderProgram,
//        buffer : ArrayBuffer,
//        drawcall:
//     */
// }

pub mod MathLayers {
    use nalgebra_glm::{Vec3, Vec4};


    /// ### Description
    /// * A structure for orthogonal coordinates, it also can map vectors using specific coordinates
    #[derive(Clone, Copy)]
    pub struct OrthogonalCoordinates {
        pub origin: Vec3,
        pub u_base: Vec3,
        pub v_base: Vec3,
        pub w_base: Vec3,
    }

    impl OrthogonalCoordinates {
        /// ### Description
        /// * create new coordinates, by giving it two axes the third is calculated using cross product
        pub fn new(origin: Vec3, U: Vec3, V: Vec3) -> Self {
            Self {
                origin: origin,
                u_base: U,
                v_base: V,
                w_base: nalgebra_glm::cross(&U, &V),
            }
        }

        pub fn new_from_normal(origin: Vec3, normal: Vec3, up: Vec3) -> Self {
            let mut right = nalgebra_glm::cross(&up, &normal);
            Self {
                origin: origin,
                u_base: right,
                v_base: up,
                w_base: normal,
            }
        }
        /// ### Description
        /// * Calculate the vector for given bases `vec`
        pub fn map(&self, vec: &Vec3) -> Vec3 {
            self.origin + vec.x * self.u_base + vec.y * self.v_base + vec.z * self.w_base
        }

        /// ### Description
        /// * Calculate the vector for given bases `vec`
        pub fn map_4f(&self, vec: &Vec4) -> Vec4 {
            let tmp = vec.x * self.u_base + vec.y * self.v_base + vec.z * self.w_base;
            Vec4::new(
                self.origin.x + tmp.x,
                self.origin.y + tmp.y,
                self.origin.z + tmp.z,
                1.0,
            )
        }
    }
    impl Default for OrthogonalCoordinates {
        fn default() -> Self {
            Self {
                origin: Vec3::zeros(),
                u_base: Vec3::x(),
                v_base: Vec3::y(),
                w_base: Vec3::z(),
            }
        }
    }

    pub struct LineLayer {}
}
