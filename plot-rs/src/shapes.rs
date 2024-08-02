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
