#[allow(non_camel_case_types)]

use gl;
extern crate num;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_f32_f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl f32_f32_f32 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        f32_f32_f32 {
            x, y, z
        }
    }

    pub unsafe fn vertex_attrib_pointer 
    (
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

impl From<(f32, f32, f32)> for f32_f32_f32 {
    fn from(other: (f32, f32, f32)) -> Self {
        f32_f32_f32::new(other.0, other.1, other.2)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u2_u10_u10_u10_rev_float {
    data: u32,
}

impl u2_u10_u10_u10_rev_float {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> u2_u10_u10_u10_rev_float {
        let x = (num::clamp(x, 0.0, 1.0) * 1023f32).round() as u32;
        let y = (num::clamp(y, 0.0, 1.0) * 1023f32).round() as u32;
        let z = (num::clamp(z, 0.0, 1.0) * 1023f32).round() as u32;
        let w = (num::clamp(w, 0.0, 1.0) * 3f32).round() as u32;

        let mut c: u32 = 0;
        c |= w << 30;
        c |= z << 20;
        c |= y << 10;
        c |= x << 0;

        u2_u10_u10_u10_rev_float {
            data: c
        }
    }

    pub unsafe fn vertex_attrib_pointer 
    (
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4,
            gl::UNSIGNED_INT_2_10_10_10_REV,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}

impl From<(f32, f32, f32, f32)> for u2_u10_u10_u10_rev_float {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        u2_u10_u10_u10_rev_float::new(other.0, other.1, other.2, other.3)
    }
}
