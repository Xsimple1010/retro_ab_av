use std::{ffi::CString, ptr::null};

use gl::{types::*, COMPILE_STATUS};

#[derive(Debug)]
pub struct Shader {
    pub program: GLuint,
    pub vao: GLuint,
    pub vbo: GLuint,
    pub i_pos: GLint,
    pub i_text_pos: GLint,
    // pub u_tex: GLint,
    // _u_mvp: GLint,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            if gl::DeleteProgram::is_loaded() {
                gl::DeleteProgram(self.program);
                gl::DeleteBuffers(1, &self.vbo);
                gl::DeleteVertexArrays(1, &self.vao);
            }
        }
    }
}

impl Shader {
    pub fn new() -> Shader {
        let vertex_shader_src = "
        #version 330 core
        in vec2 i_pos;
        in vec2 t_pos;

        out vec2 f_t_pos;

        void main() {
            f_t_pos = t_pos;
            gl_Position = vec4(i_pos, 0.0, 1.0);
        }
        ";

        let fragment_shader_src = "
        #version 330 core
        in vec2 f_t_pos;
        
        out vec4 FragColor;
        
        uniform sampler2D u_tex;

        void main() {
            FragColor = texture2D(u_tex, f_t_pos);
        }
        ";

        let vertex_shader = _compile(gl::VERTEX_SHADER, vertex_shader_src);
        let fragment_shader = _compile(gl::FRAGMENT_SHADER, fragment_shader_src);
        let program = unsafe { gl::CreateProgram() };
        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            gl::ValidateProgram(program);

            let mut status = 0;

            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

            if status == 0 {
                let log = CString::new("").unwrap();
                let log_ptr = log.into_raw();
                let mut length = 0;

                gl::GetProgramInfoLog(program, 4096, &mut length, log_ptr);

                let log = CString::from_raw(log_ptr);

                panic!("{:?}", log);
            }

            let param_name = CString::new("i_pos").unwrap();
            let i_pos = gl::GetAttribLocation(program, param_name.as_ptr());

            let param_name = CString::new("t_pos").unwrap();
            let i_text_pos = gl::GetAttribLocation(program, param_name.as_ptr());

            let param_name = CString::new("u_tex").unwrap();
            let u_tex = gl::GetUniformLocation(program, param_name.as_ptr());

            // let param_name = CString::new("u_mvp").unwrap();
            // self.u_mvp = gl::GetUniformLocation(program, param_name.as_ptr());

            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::UseProgram(program);
            gl::Uniform1i(u_tex, 0);
            // if gl::

            gl::UseProgram(0);

            Shader {
                program,
                vao,
                vbo,
                i_text_pos,
                i_pos, // texture_coords,
                       // u_tex,
                       // _u_mvp: 0,
            }
        }
    }
}

fn _compile(shader_type: GLenum, source_code: &str) -> GLuint {
    unsafe {
        let shader = gl::CreateShader(shader_type);

        let source = CString::new(source_code).expect("erro ao criar um c string");
        let source = source.as_c_str().as_ptr();

        gl::ShaderSource(shader, 1, &source, null());
        gl::CompileShader(shader);

        let mut status = 0;
        gl::GetShaderiv(shader, COMPILE_STATUS, &mut status);

        if status == 0 {
            let log = CString::new("").unwrap();
            let log_ptr = log.into_raw();
            let mut length = 0;

            gl::GetShaderInfoLog(shader, 4096, &mut length, log_ptr);

            let log = CString::from_raw(log_ptr);

            panic!("{:?}", log);
        }

        shader
    }
}
