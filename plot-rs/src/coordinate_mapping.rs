use nalgebra_glm::{Mat4, Vec2, Vec3, Vec4};

#[derive(Clone, Copy, Debug)]
pub struct Coordinate {
    // U: Vec3,
    // V:Vec3,
    // W:Vec3,
    mapping_transformation: Mat4,
}
impl Default for Coordinate {
    fn default() -> Self {
        Self {
            /*
            U:Vec3::x(),
            V:Vec3::y(),
            W:Vec3::z(), */
            mapping_transformation: Mat4::identity(),
        }
    }
}
impl Coordinate {
    pub fn new_from_basis(x: Vec3, y: Vec3, z: Vec3) -> Self {
        let mut mat = Mat4::zeros();
        mat[0] = x.x;
        mat[1] = x.y;
        mat[2] = x.z;

        mat[4] = y.x;
        mat[5] = y.y;
        mat[6] = y.z;

        mat[8] = z.x;
        mat[9] = z.y;
        mat[10] = z.z;

        mat[15] = 1.0;

        Self {
            mapping_transformation: mat,
        }
    }

    fn get_consecutive_elements_vector(&self, idx: usize) -> Vec3 {
        Vec3::new(
            self.mapping_transformation[idx],
            self.mapping_transformation[idx + 1],
            self.mapping_transformation[idx + 2],
        )
    }
    pub fn get_U(&self) -> Vec3 {
        self.get_consecutive_elements_vector(0) // Vec3::new(self.mapping_transformation[0], self.mapping_transformation[1], self.mapping_transformation[2])
    }
    pub fn get_V(&self) -> Vec3 {
        self.get_consecutive_elements_vector(4)
    }
    pub fn get_W(&self) -> Vec3 {
        self.get_consecutive_elements_vector(8)
    }
    pub fn get_T(&self) -> Vec3 {
        self.get_consecutive_elements_vector(12)
    }

    pub fn map_4d(&self, v: &Vec4) -> Vec4 {
        self.mapping_transformation * v
    }
    pub fn map_3d(&self, v: &Vec3) -> Vec4 {
        self.mapping_transformation * Vec4::new(v.x, v.y, v.z, 1.0)
    }
    pub fn map_2d(&self, v: &Vec2) -> Vec4 {
        self.mapping_transformation * Vec4::new(v.x, v.y, 0.0, 1.0)
    }

    pub fn rotate_ccw_u(&mut self, degrees: f32) -> &mut Self {
        self.mapping_transformation =
            nalgebra_glm::rotate(&self.mapping_transformation, degrees, &self.get_U());

        self
    }

    pub fn rotate_ccw_v(&mut self, degrees: f32) -> &mut Self {
        self.mapping_transformation =
            nalgebra_glm::rotate(&self.mapping_transformation, degrees, &self.get_V());

        self
    }

    pub fn rotate_ccw_w(&mut self, degrees: f32) -> &mut Self {
        self.mapping_transformation =
            nalgebra_glm::rotate(&self.mapping_transformation, degrees, &self.get_W());

        self
    }

    pub fn set_translation(&mut self, t: Vec3) -> &mut Self {
        self.mapping_transformation[12] = t.x;
        self.mapping_transformation[13] = t.y;
        self.mapping_transformation[14] = t.z;
        self
    }
    pub fn translate(&mut self, t: Vec3) -> &mut Self {
        self.mapping_transformation[12] += t.x;
        self.mapping_transformation[13] += t.y;
        self.mapping_transformation[14] += t.z;
        self
    }
    pub fn scale(
        &mut self,
        x_scale: Option<f32>,
        y_scale: Option<f32>,
        z_scale: Option<f32>,
    ) -> &mut Self {
        if let Some(xs) = x_scale {
            self.mapping_transformation[0] *= xs;
            self.mapping_transformation[1] *= xs;
            self.mapping_transformation[2] *= xs;
        }

        if let Some(ys) = y_scale {
            self.mapping_transformation[4] *= ys;
            self.mapping_transformation[5] *= ys;
            self.mapping_transformation[6] *= ys;
        }

        if let Some(zs) = z_scale {
            self.mapping_transformation[8] *= zs;
            self.mapping_transformation[9] *= zs;
            self.mapping_transformation[10] *= zs;
        }

        self
    }

    pub fn map_vectors_inplace(&self, vectors: &mut [Vec3]) -> &Self {
        for old_vec in vectors.iter_mut() {
            *old_vec = self.map_3d(&*old_vec).xyz();
        }
        self
    }
    // pub fn map_vector_elements_inplace<TYPE,ELEM_VEC_MAP_FUN>(&self,func:ELEM_VEC_MAP_FUN)->&Self

    // where ELEM_VEC_MAP_FUN : FnMut()

    // {

    //     self
    // }
}
