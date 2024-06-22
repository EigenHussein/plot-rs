use std::mem::size_of;

#[cfg(debug_assertions)]
fn print_error(msg: &'static str) {
    println!("{}", msg);
}
#[cfg(not(debug_assertions))]
fn print_error(msg: &'static str) {}

pub struct GLAttrib {
    // valid: bool,
    attribute_index: u32,
    size: gl::types::GLint,
    type_: gl::types::GLenum,
    normalized: gl::types::GLboolean,
    stride: gl::types::GLsizei,
    pointer: gl::types::GLuint,
}

impl GLAttrib {
    pub fn set(&self) {
        unsafe {
            gl::VertexAttribPointer(
                self.attribute_index,
                self.size,
                self.type_,
                self.normalized,
                self.stride,
                self.pointer as _,
            );
            gl::EnableVertexAttribArray(self.attribute_index);
        }
    }
}
impl Default for GLAttrib {
    fn default() -> Self {
        GLAttrib {
            attribute_index: 0,
            size: 0,
            type_: gl::FLOAT,
            normalized: 0,
            stride: 0,
            pointer: 0,
        }
    }
}
impl Clone for GLAttrib {
    fn clone(&self) -> Self {
        GLAttrib {
            attribute_index: self.attribute_index,
            size: self.size,
            type_: self.type_,
            normalized: self.normalized,
            stride: self.stride,
            pointer: self.pointer,
        }
    }
}
impl GLAttrib {
    pub fn new(idx: u32, size: i32, stride: i32, offset: u32) -> Self {
        GLAttrib {
            attribute_index: idx,
            size: size,
            type_: gl::FLOAT,
            normalized: 0,
            stride: stride * (std::mem::size_of::<f32>() as i32),
            pointer: offset * (std::mem::size_of::<f32>() as u32),
        }
    }
}
pub struct VAO {
    vao: u32,
    attributes: Vec<GLAttrib>,
}
impl VAO {
    fn new() -> Self {
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        Self {
            vao: vao,
            attributes: Vec::new(),
        }
    }
    fn set_attributes(&mut self, attribs: &Vec<GLAttrib>) -> &mut Self {
        self.attributes = attribs.clone();
        self
    }
    fn bind_attributes_to_vao(&mut self) -> &mut Self {
        self.bind();
        for attr in self.attributes.iter() {
            attr.set();
        }
        self
    }
    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.vao) };
    }
    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0) }
    }
}
impl Drop for VAO {
    fn drop(&mut self) {
        print_error("Dropped VAO");
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}

pub struct VBO {
    vbo: Option<u32>,
    buffer_type: u32,
    element_size: usize,
    element_count: usize,
    buffer_size: usize,
}
impl Default for VBO {
    fn default() -> Self {
        Self {
            vbo: None,
            buffer_type: 0,
            element_size: 0,
            element_count: 0,
            buffer_size: 0,
        }
    }
}
impl VBO {
    pub fn array_buffer_from_slice<VertexStructure>(
        vertices: &[VertexStructure],
        static_buffer: bool,
    ) -> Self {
        let number_of_vertices = vertices.len();
        let size_of_vertex = size_of::<VertexStructure>();
        let vertex_ptr = vertices.as_ptr();

        let mut vbo = 0u32;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (number_of_vertices * size_of_vertex) as _,
                vertex_ptr as _,
                if static_buffer == true {
                    gl::STATIC_DRAW
                } else {
                    gl::DYNAMIC_DRAW
                },
            );
        }

        Self {
            buffer_type: gl::ARRAY_BUFFER,
            vbo: Some(vbo),
            element_count: number_of_vertices,
            element_size: size_of_vertex,
            buffer_size: size_of_vertex * number_of_vertices,
        }
    }
    pub fn allocate_array_buffer(
        target: Option<u32>,
        buffer_size_in_bytes: usize,
        static_buffer: bool,
    ) -> Self {
        let mut vbo = 0u32;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(target.unwrap_or(gl::ARRAY_BUFFER), vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                buffer_size_in_bytes as _,
                0 as _,
                if static_buffer == true {
                    gl::STATIC_DRAW
                } else {
                    gl::DYNAMIC_DRAW
                },
            );
        }

        Self {
            buffer_type: gl::ARRAY_BUFFER,
            vbo: Some(vbo),
            element_size: 1,
            element_count: buffer_size_in_bytes,
            buffer_size: buffer_size_in_bytes,
        }
    }
    pub fn update_buffer_slice<VertexStructure>(
        &mut self,
        element_offset: usize,
        new_vertices: &[VertexStructure],
    ) -> &mut Self {
        let update_size = if element_offset + new_vertices.len() > self.element_count {
            println!("Warning: buffer's is not large enough ");
            (self.element_count - element_offset) * self.element_size
        } else {
            new_vertices.len() * self.element_size
        };

        unsafe {
            gl::NamedBufferSubData(
                self.vbo.expect("None intialized VBO"),
                (element_offset * self.element_size) as _,
                (update_size * self.element_size) as _,
                new_vertices.as_ptr() as _,
            )
        }

        self
    }
    pub fn bind(&self) {
        unsafe { gl::BindBuffer(self.buffer_type, self.vbo.expect("None intialized VBO")) };
    }
    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(self.buffer_type, 0) }
    }
}

impl Drop for VBO {
    fn drop(&mut self) {
        print_error("Dropped VBO!");
        unsafe { gl::DeleteBuffers(1, &self.vbo.expect("None intialized VBO")) }
    }
}

pub struct EBO {
    ebo: Option<u32>,
    n_indices: usize,
}
impl Default for EBO {
    fn default() -> Self {
        Self {
            ebo: None,
            n_indices: 0,
        }
    }
}

impl EBO {
    pub fn new(indices: &[u32], static_buffer: bool) -> Self {
        let mut ebo = 0u32;
        unsafe {
            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * size_of::<u32>()) as _,
                indices.as_ptr() as _,
                if static_buffer == true {
                    gl::STATIC_DRAW
                } else {
                    gl::DYNAMIC_DRAW
                },
            );
        }

        Self {
            ebo: Some(ebo),
            n_indices: indices.len(),
        }
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(
                gl::ELEMENT_ARRAY_BUFFER,
                self.ebo.expect("None intialized EBO"),
            )
        };
    }
    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0) }
    }
}
impl Drop for EBO {
    fn drop(&mut self) {
        if self.ebo.is_some() {
            print_error("Dropped EBO!");
            unsafe {
                gl::DeleteBuffers(1, &self.ebo.unwrap());
            }
        }
    }
}

pub struct ArrayBuffer {
    pub vao: VAO,
    pub vbo: VBO,
    pub ebo: EBO,
}

impl ArrayBuffer {
    pub fn default_buffer_from_slice<VertexStructure>(
        vertices: &[VertexStructure],
        indices: Option<&[u32]>,
        buffer_data_attributes: &Vec<GLAttrib>,
        static_buffer: bool,
    ) -> Self {
        let mut vao = VAO::new();

        let vbo = VBO::array_buffer_from_slice(vertices, static_buffer);

        vao.set_attributes(buffer_data_attributes)
            .bind_attributes_to_vao();

        let ebo = if let Some(valid_indices) = indices {
            Some(EBO::new(valid_indices, static_buffer))
        } else {
            None
        };

        ArrayBuffer {
            vao: vao,
            vbo: vbo,
            ebo: ebo.unwrap_or_default(),
        }
    }
}
