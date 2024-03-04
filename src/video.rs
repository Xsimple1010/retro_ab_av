use crate::retro_gl::{self, shader::Shader};
use retro_ab::core::AvInfo;
use std::{
    ffi::c_uint,
    os::raw::c_void,
    ptr::{null, slice_from_raw_parts},
    sync::Arc,
};
use winit::{error::EventLoopError, event_loop::EventLoop};

static mut RAW_TEX_POINTER: NextFrame = NextFrame {
    _data: null(),
    _pitch: 0,
    _height: 0,
    _width: 0,
    _slice: [[0 as i16]].as_ptr(),
};

pub fn video_refresh_callback(
    _data: *const c_void,
    _width: c_uint,
    _height: c_uint,
    _pitch: usize,
) {
    unsafe {
        let _slice = slice_from_raw_parts(_data as *const i16, (_width * _height) as usize);

        RAW_TEX_POINTER = NextFrame {
            _data,
            _height,
            _width,
            _pitch,
            _slice,
        };
    }
}

struct NextFrame {
    _data: *const c_void,
    _width: c_uint,
    _height: c_uint,
    _pitch: usize,
    _slice: *const [i16],
}

pub struct RetroVideo {
    pub window: winit::window::Window,
    shader: retro_gl::shader::Shader,
}

impl RetroVideo {
    pub fn draw_new_frame(&mut self, _av_info: &Arc<AvInfo>) {
        //isso resolve o uso exagerado de memoria ram
        for _ in 0..38_900_00 {}
    }
    pub fn resize(&mut self, _new_size: (u32, u32)) {}
}

pub fn init(_av_info: &Arc<AvInfo>) -> Result<(RetroVideo, EventLoop<()>), EventLoopError> {
    let event_loop = EventLoop::new()?;
    let window = winit::window::Window::new(&event_loop).expect("erro ao tentar cria um janela");

    //TODO: carregar as funções do opengl primeiro

    let mut shader = Shader::default();
    shader.init();

    Ok((RetroVideo { window, shader }, event_loop))
}
