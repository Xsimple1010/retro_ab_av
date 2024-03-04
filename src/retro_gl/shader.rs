use std::ffi::CString;

use gl::{types::*, COMPILE_STATUS};

#[derive(Debug, Default)]
pub struct Shader {
    program: GLuint,
    fragment_shader: GLuint,
    vertex_shader: GLuint,
    vao: GLuint,
    vbo: GLuint,
    position: GLint,
    texture_coords: GLint,
    u_tex: GLint,
    u_mvp: GLint,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            if gl::DeleteProgram::is_loaded() {
                gl::DeleteProgram(self.program);
            }
        }
    }
}

impl Shader {
    fn _compile(&self, shader_type: GLenum, source: &str) -> GLuint {
        unsafe {
            let shader = gl::CreateShader(shader_type);

            let source = CString::new(source).expect("erro ao criar um c string");
            let source = source.as_c_str().as_ptr();
            let len = 0;

            gl::ShaderSource(shader, 1, &source, &len);

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

    pub fn init(&mut self) {
        let vertex_shader_src = "
        #version 330 core
        in vec2 position;
        in vec2 texture_coords;
        
        out vec2 v_tex_coords;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
            
            v_tex_coords = texture_coords;
        }
        ";

        let fragment_shader_src = "
        #version 330 core
        in vec2 v_tex_coords;
        out vec4 FragColor;

        uniform sampler2D u_tex;

        void main() {
            FragColor = texture(u_tex, v_tex_coords);
        }
        ";

        self.vertex_shader = self._compile(gl::VERTEX_SHADER, vertex_shader_src);
        self.fragment_shader = self._compile(gl::FRAGMENT_SHADER, fragment_shader_src);
        self.program = unsafe { gl::CreateProgram() };

        unsafe {
            gl::AttachShader(self.program, self.vertex_shader);
            gl::AttachShader(self.program, self.fragment_shader);
            gl::LinkProgram(self.program);

            gl::DeleteShader(self.vertex_shader);
            gl::DeleteShader(self.fragment_shader);

            gl::ValidateProgram(self.program);

            let mut status = 0;

            gl::GetProgramiv(self.program, gl::LINK_STATUS, &mut status);

            if status == 0 {
                let log = CString::new("").unwrap();
                let log_ptr = log.into_raw();
                let mut length = 0;

                gl::GetProgramInfoLog(self.program, 4096, &mut length, log_ptr);

                let log = CString::from_raw(log_ptr);

                panic!("{:?}", log);
            }

            let param_name = CString::new("position").unwrap();
            self.position = gl::GetAttribLocation(self.program, param_name.as_ptr());

            let param_name = CString::new("texture_coords").unwrap();
            self.texture_coords = gl::GetAttribLocation(self.program, param_name.as_ptr());

            let param_name = CString::new("u_tex").unwrap();
            self.u_tex = gl::GetUniformLocation(self.program, param_name.as_ptr());

            let param_name = CString::new("u_mvp").unwrap();
            self.u_mvp = gl::GetUniformLocation(self.program, param_name.as_ptr());

            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);

            gl::UseProgram(self.program);
            gl::Uniform1i(self.u_tex, 0);
            // if gl::

            gl::UseProgram(0);
        }
    }
}
