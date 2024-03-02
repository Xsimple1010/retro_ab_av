use glium::{
    backend::glutin::SimpleWindowBuilder, glutin::surface::WindowSurface, implement_vertex,
    uniform, Display, IndexBuffer, Program, Surface, VertexBuffer,
};
use retro_ab::core::AvInfo;
use std::{ffi::c_uint, os::raw::c_void, sync::Arc};
use winit::{
    error::EventLoopError,
    event_loop::{EventLoop, EventLoopBuilder},
    window::Window,
};

//
// static mut RAW_TEX_POINTER: *const c_void = null() as *const c_void;

pub fn video_refresh_callback(
    _data: *const c_void,
    _width: c_uint,
    _height: c_uint,
    _pitch: usize,
) {
}

pub struct RetroVideo {
    pub window: Window,
    _display: Display<WindowSurface>,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
    program: Program,
    // texture: Texture2d,
}

impl RetroVideo {
    pub fn draw_new_frame(&mut self) {
        let mut frame = self._display.draw();
        let uniforms = uniform! {matrix :[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ]};

        frame.clear_color(0., 0., 0., 0.);

        frame
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
                &self.program,
                &uniforms,
                &Default::default(),
            )
            .expect("erro ao tentar desenha o triangulo");

        frame
            .finish()
            .expect("erro ao tentar desenha um novo frame");
    }

    pub fn resize(&mut self, new_size: (u32, u32)) {
        self._display.resize(new_size);
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    // color: [f32; 4],
}

implement_vertex!(Vertex, position);

pub fn init(_av_info: &Arc<AvInfo>) -> Result<(RetroVideo, EventLoop<()>), EventLoopError> {
    let event_loop = EventLoopBuilder::new().build()?;
    let (window, _display) = SimpleWindowBuilder::new().build(&event_loop);

    let vertex_buffer = {
        glium::VertexBuffer::new(
            &_display,
            &[
                Vertex {
                    position: [-0.5, -0.5],
                    // color: [0.0, 1.0, 0.0, 1.0],
                },
                Vertex {
                    position: [0.0, 0.5],
                    // color: [0.0, 0.0, 1.0, 1.0],
                },
                Vertex {
                    position: [0.5, -0.5],
                    // color: [1.0, 0.0, 0.0, 1.0],
                },
            ],
        )
        .unwrap()
    };

    let index_buffer = IndexBuffer::new(
        &_display,
        glium::index::PrimitiveType::TrianglesList,
        &[0u16, 1, 2],
    )
    .unwrap();

    let program = Program::from_source(
        &_display,
        "
        #version 330 core
        in vec2 position;
        uniform mat4 matrix;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0) * matrix;
        }
        ",
        "
        #version 330 core
        out vec4 FragColor;
        void main() {
            FragColor = vec4(0.8f, 0.3f, 0.2f, 1.0f);
        }
        ",
        None,
    )
    .expect("erro ao tentar cria shader program");

    Ok((
        RetroVideo {
            _display,
            window,
            program,
            vertex_buffer,
            index_buffer,
        },
        event_loop,
    ))
}
