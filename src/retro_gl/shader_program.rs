use std::ffi::CString;

use gl::types::*;

use super::shader::Shader;

pub struct ShaderProgram {
    id: GLuint,
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl ShaderProgram {
    pub fn new(shaders: &[Shader]) -> ShaderProgram {
        let mut _id = 0;

        unsafe {
            _id = gl::CreateProgram();

            for shader in shaders {
                gl::AttachShader(_id, shader.id);
            }

            gl::LinkProgram(_id);
            gl::ValidateProgram(_id);

            let mut status = 0;

            gl::GetProgramiv(_id, gl::LINK_STATUS, &mut status);

            if status == 0 {
                let mut error_log_size: GLint = 0;
                gl::GetProgramiv(_id, gl::INFO_LOG_LENGTH, &mut error_log_size);
                let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
                gl::GetProgramInfoLog(
                    _id,
                    error_log_size,
                    &mut error_log_size,
                    error_log.as_mut_ptr() as *mut _,
                );

                error_log.set_len(error_log_size as usize);
                let log = String::from_utf8(error_log).expect("msg");

                println!("{:?}", log);
            }
        }

        Self { id: _id }
    }

    pub fn get_attribute(&self, name: &str) -> GLint {
        let param_name = CString::new(name).unwrap();
        unsafe { gl::GetAttribLocation(self.id, param_name.as_ptr()) }
    }

    pub fn get_uniform(&self, name: &str) -> GLint {
        let param_name = CString::new(name).unwrap();
        unsafe { gl::GetUniformLocation(self.id, param_name.as_ptr()) }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn un_use_program(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }
}
