use super::{
    shader::Shader,
    texture::{RawTextureData, Texture2D, TexturePosition},
};
use gl::{self, types::GLuint};
use retro_ab::core::AvInfo;
use std::{
    mem::{size_of, size_of_val},
    sync::Arc,
};

pub struct Render {
    pub shader: Shader,
    texture: Texture2D,
    av_info: Arc<AvInfo>,
}

type Pos = [f32; 2];

#[repr(C, packed)]
struct Vertex(Pos, TexturePosition);
impl Render {
    fn refresh_vertex(&self) {
        unsafe {
            let geo = &self.av_info.video.geometry;

            let bottom =
                *geo.base_width.lock().unwrap() as f32 / *geo.max_width.lock().unwrap() as f32;
            let right =
                *geo.base_height.lock().unwrap() as f32 / *geo.max_height.lock().unwrap() as f32;

            let vertices: [Vertex; 4] = [
                Vertex([-1.0, -1.0], [0.0, bottom]),
                Vertex([-1.0, 1.0], [0.0, 0.0]),
                Vertex([1.0, -1.0], [right, bottom]),
                Vertex([1.0, 1.0], [right, 0.0]),
            ];

            gl::BindVertexArray(self.shader.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.shader.vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                size_of_val(&vertices) as isize,
                vertices.as_ptr().cast(),
                gl::STREAM_DRAW,
            );

            gl::EnableVertexAttribArray(self.shader.i_pos as GLuint);
            gl::VertexAttribPointer(
                self.shader.i_pos as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                std::ptr::null(),
            );

            gl::EnableVertexAttribArray(self.shader.i_text_pos as GLuint);
            gl::VertexAttribPointer(
                self.shader.i_text_pos as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                (size_of::<f32>() * 2) as *const _,
            );

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn draw_new_frame(&self, next_frame: &RawTextureData) {
        unsafe {
            self.refresh_vertex();

            self.texture.push(next_frame);

            gl::Viewport(0, 0, next_frame.width as i32, next_frame.height as i32);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(self.shader.program);
            self.texture.active();

            gl::BindVertexArray(self.shader.vao);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);

            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
    }

    pub fn new(av_info: &Arc<AvInfo>) -> Result<Render, String> {
        let texture = Texture2D::new(av_info)?;

        Ok(Render {
            shader: Shader::new(),
            texture,
            av_info: av_info.clone(),
        })
    }
}
