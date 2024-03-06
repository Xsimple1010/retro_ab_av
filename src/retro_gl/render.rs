use core::ffi::{c_uint, c_void};
use gl::{self, types::GLuint};
use retro_ab::core::AvInfo;
use std::{
    mem::{size_of, size_of_val},
    ptr::null,
    sync::Arc,
};

use super::{pixel::Pixel, shader::Shader};

pub struct NextFrame {
    pub _data: *const c_void,
    pub _width: c_uint,
    pub _height: c_uint,
    pub _pitch: usize,
}

pub struct Render {
    pub shader: Shader,
    texture: GLuint,
    pixel: Pixel,
    av_info: Arc<AvInfo>,
}

impl Drop for Render {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.texture) }
    }
}

type Pos = [f32; 2];
type TexPos = [f32; 2];
#[repr(C, packed)]
struct Vertex(Pos, TexPos);
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
                0 as *const _,
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

    pub fn draw_new_frame(&self, next_frame: &NextFrame) {
        unsafe {
            self.refresh_vertex();

            let len = next_frame._pitch as i32 / self.pixel.bpm;
            let wid = next_frame._width as i32;
            let he = next_frame._height as i32;

            gl::BindTexture(gl::TEXTURE0, self.texture);
            gl::PixelStorei(gl::UNPACK_ROW_LENGTH, len);
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                0,
                0,
                wid,
                he,
                self.pixel.typ,
                self.pixel.format,
                next_frame._data,
            );
            gl::Viewport(0, 0, wid, he);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(self.shader.program);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);

            gl::BindVertexArray(self.shader.vao);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);

            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
    }

    pub fn new(av_info: &Arc<AvInfo>) -> Result<Render, String> {
        let mut texture = 0;
        let pixel = Pixel::new(&av_info.video.pixel_format.lock().unwrap())?;

        unsafe {
            let geo = &av_info.video.geometry;

            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA8 as i32,
                *geo.max_height.lock().unwrap() as i32,
                *geo.max_width.lock().unwrap() as i32,
                0,
                pixel.typ,
                pixel.format,
                null(),
            );

            gl::BindTexture(gl::TEXTURE_2D, 0)
        }

        Ok(Render {
            shader: Shader::new(),
            pixel,
            texture,
            av_info: av_info.clone(),
        })
    }
}
