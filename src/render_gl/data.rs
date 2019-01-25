use gl;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        vec3f {
            x, y, z
        }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl, 
        stride: usize, 
        location: usize,
        offset: usize
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}

impl From<(f32, f32, f32)> for vec3f {
    fn from(other: (f32, f32, f32)) -> Self {
        vec3f::new(other.0, other.1, other.2)
    }
}
