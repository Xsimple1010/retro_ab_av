use super::{
    gl::gl::{
        self,
        types::{GLint, GLuint},
    },
    gl_buffer::GlBuffer,
    shader::Shader,
    shader_program::ShaderProgram,
    texture::{RawTextureData, Texture2D},
    vertex::{new_vertex, GlVertex},
    vertex_array::VertexArray,
};
use retro_ab::core::{AvInfo, Geometry};
use std::mem::size_of;
use std::{rc::Rc, sync::Arc};

pub struct Render {
    _program: ShaderProgram,
    _texture: Texture2D,
    _i_pos: GLint,
    _i_tex_pos: GLint,
    _u_tex: GLint,
    _vao: VertexArray,
    _vbo: GlBuffer,
    gl: Rc<gl::Gl>,
}

impl Render {
    fn refresh_vertex(
        &self,
        geo: &Geometry,
        origin_w: f32,
        origin_h: f32,
        window_w: i32,
        window_h: i32,
    ) {
        let vertex = new_vertex(geo, window_w as f32, window_h as f32, origin_w, origin_h);

        self._vao.bind();
        self._vbo.bind();

        self._vbo.set_data(vertex);
        self._vao
            .set_attribute::<GlVertex>(self._i_pos as GLuint, 2, 0);

        self._vao.set_attribute::<GlVertex>(
            self._i_tex_pos as GLuint,
            2,
            (size_of::<f32>() * 2) as i32,
        );

        self._vao.un_bind();
        self._vbo.un_bind();
    }

    pub fn draw_new_frame(
        &self,
        next_frame: &RawTextureData,
        geo: &Geometry,
        win_width: i32,
        win_height: i32,
    ) {
        self.refresh_vertex(
            geo,
            next_frame.width as f32,
            next_frame.height as f32,
            win_width,
            win_height,
        );

        unsafe {
            self.gl.Viewport(0, 0, win_width, win_height);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);

            self._texture.push(next_frame);
            self._program.use_program();
            self._texture.active();

            self._vao.bind();
            self.gl.DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
            self._vao.un_bind();
            self._program.un_use_program();
        }
    }

    pub fn new(av_info: &Arc<AvInfo>, gl: Rc<gl::Gl>) -> Result<Render, String> {
        let vertex_shader_src = "
        #version 330 core
        in vec2 i_pos;
        in vec2 i_tex_pos;

        out vec2 f_t_pos;

        void main() {
            f_t_pos = i_tex_pos;
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

        let vertex_shader = Shader::new(gl::VERTEX_SHADER, vertex_shader_src, gl.clone());
        let frag_shader = Shader::new(gl::FRAGMENT_SHADER, fragment_shader_src, gl.clone());

        let program = ShaderProgram::new(&[vertex_shader, frag_shader], gl.clone());

        let i_pos = program.get_attribute("i_pos");
        let i_tex_pos = program.get_attribute("i_tex_pos");
        let u_tex = program.get_uniform("u_tex");

        let texture = Texture2D::new(av_info, gl.clone())?;

        let vao = VertexArray::new(gl.clone());
        let vbo = GlBuffer::new(gl::ARRAY_BUFFER, gl.clone());

        Ok(Render {
            _program: program,
            _texture: texture,
            _i_pos: i_pos,
            _i_tex_pos: i_tex_pos,
            _u_tex: u_tex,
            _vao: vao,
            _vbo: vbo,
            gl,
        })
    }
}
