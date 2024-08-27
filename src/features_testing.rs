use nalgebra_glm::{Mat4, Vec3, Vec4};
use plot_rs::coordinate_mapping::Coordinate;
fn main() {
    let coords = Coordinate::new_from_basis(1.0 * Vec3::x(), 2.0 * Vec3::y(), 3.0 * Vec3::z());

    let test_vec = Vec4::new(1.0, 1.0, 1.0, 1.0);
    println!("{:?}", coords.map_4d(&test_vec));
}
