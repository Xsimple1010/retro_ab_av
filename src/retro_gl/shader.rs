use gl::COMPILE_STATUS;
use std::{ffi::CString, ptr::null};

use super::gl::gl::{
    self,
    types::{GLenum, GLuint},
};

#[derive(Debug)]
pub struct Shader {
    // pub program: GLuint,
    // pub vao: GLuint,
    // pub vbo: GLuint,
    // pub i_pos: GLint,
    // pub i_text_pos: GLint,
    pub id: GLuint,
    // pub u_tex: GLint,
    // _u_mvp: GLint,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            if gl::DeleteShader::is_loaded() {
                gl::DeleteShader(self.id);
            }
        }
    }
}

impl Shader {
    pub fn new(shader_type: GLenum, source_code: &str) -> Shader {
        unsafe {
            let id = gl::CreateShader(shader_type);

            let source = CString::new(source_code).expect("erro ao criar um c string");
            let source = source.as_c_str().as_ptr();

            gl::ShaderSource(id, 1, &source, null());
            gl::CompileShader(id);

            let mut status = 0;
            gl::GetShaderiv(id, COMPILE_STATUS, &mut status);

            if status == 0 {
                let log = CString::new("").unwrap();
                let log_ptr = log.into_raw();
                let mut length = 0;

                gl::GetShaderInfoLog(id, 4096, &mut length, log_ptr);

                let log = CString::from_raw(log_ptr);

                panic!("{:?}", log);
            }

            Self { id }
        }
    }
}
