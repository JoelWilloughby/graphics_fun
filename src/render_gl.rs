use gl;
use crate::resources::{self, Resources};
use std;
use std::ffi::{CString, CStr};

#[derive(Debug)]
pub enum Error {
    ResourceLoad { inner: resources::Error },
    CompileError { message: String },
    LinkError { message: String },
}

impl From<resources::Error> for Error {
    fn from(other: resources::Error) -> Self {
        Error::ResourceLoad{inner: other}
    }
}

pub trait ShaderSource {
    fn fragment_shader(&self) -> &String;
    fn vertex_shader(&self) -> &String;
}

pub struct ShaderCollection {
    fragment: String,
    vertex: String,
}

impl ShaderCollection {
    pub fn from_all(
        vertex: &str,
        fragment: &str,
    ) -> ShaderCollection {
        ShaderCollection{fragment: fragment.to_string(), vertex: vertex.to_string()}
    }

    pub fn from_simple(
        prefix: &str
    ) -> ShaderCollection {
        let vert = format!("{}.vert", prefix);
        let frag = format!("{}.frag", prefix);

        ShaderCollection{fragment: frag.to_string(), vertex: vert.to_string()}
    }
}

impl ShaderSource for ShaderCollection {
    fn fragment_shader(&self) -> &String {
        &self.fragment
    }

    fn vertex_shader(&self) -> &String {
        &self.vertex
    }
}

pub struct Program {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    buffer.extend([b' '].iter().cycle().take(len as usize));
    unsafe { CString::from_vec_unchecked(buffer) }
}

fn shader_from_source(
    gl: &gl::Gl,
    source: &CStr,
    kind: gl::types::GLenum
) -> Result<gl::types::GLuint, Error> {

    // Returns a shader id from source shader
    let id = unsafe { gl.CreateShader(kind) };

    // Try to compile the shader
    unsafe {
        gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl.CompileShader(id);
    }

    // Check for successful shader compilation
    let mut success: gl::types::GLint = 1;
    unsafe {
        gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        // Error
        let mut len : gl::types::GLint = 0;
        unsafe {
            gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        // Build up room for an error string
        let error: CString = create_whitespace_cstring_with_len(len as usize);

        // Make the error string
        unsafe {
            gl.GetShaderInfoLog(
                id, 
                len, 
                std::ptr::null_mut(), 
                error.as_ptr() as *mut gl::types::GLchar);
        }

        // Return a new owned version of the error string
        return Err(Error::CompileError{message: error.to_string_lossy().into_owned()});
    }

    Ok(id)
}

pub struct Shader {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(
        gl: &gl::Gl,
        source: &CStr,
        kind: gl::types::GLenum
    ) -> Result<Shader, Error> {
        let id = shader_from_source(gl, source, kind)?;
        Ok(Shader{gl: gl.clone(), id: id})
    }

    pub fn from_res(
        gl: &gl::Gl,
        res: &Resources,
        name: &str,
        kind: gl::types::GLenum
    ) -> Result<Shader, Error> {
        let source = res.load_cstring(name)?;

        Shader::from_source(gl, &source, kind)
    }

    pub fn from_vertex_source(gl: &gl::Gl, source: &CStr) -> Result<Shader, Error> {
        Shader::from_source(gl, source, gl::VERTEX_SHADER)
    }

    pub fn from_fragment_source(gl: &gl::Gl, source: &CStr) -> Result<Shader, Error> {
        Shader::from_source(gl, source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}

impl Program {
    pub fn from_shaders(gl: &gl::Gl, shaders: &[Shader]) -> Result<Program, Error> {
        let program_id = unsafe { gl.CreateProgram() };

        for shader in shaders {
            unsafe { gl.AttachShader(program_id, shader.id()) };
        }

        unsafe { gl.LinkProgram(program_id) };

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl.GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl.GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }

            return Err(Error::LinkError{message: error.to_string_lossy().into_owned()});
        }

        for shader in shaders {
            unsafe { gl.DetachShader(program_id, shader.id()) };
        }

        Ok(Program {gl: gl.clone(), id: program_id})
    }

    pub fn from_res<I> (
        gl: &gl::Gl,
        res: &Resources,
        collection: &I
    ) -> Result<Program, Error> where I: ShaderSource  {
        let names = [(collection.vertex_shader(), gl::VERTEX_SHADER), (collection.fragment_shader(), gl::FRAGMENT_SHADER)];

        let shaders = names.iter()
            .map(|(file, kind)| {Shader::from_res(gl, res, &file, *kind)})
            .collect::<Result<Vec<Shader>, Error>>()?;

        Program::from_shaders(gl, &shaders[..])
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe{ self.gl.UseProgram(self.id) };
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}

