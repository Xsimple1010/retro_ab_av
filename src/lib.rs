extern crate sdl2;
mod audio;
mod vao;
mod window;

use retro_ab::core::AvInfo;
use std::sync::Arc;
use window::{RetroAVInstance, RetroAvCtx};

pub use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;
pub use sdl2::EventPump;

pub fn audio_sample_callback(_left: i16, _right: i16) {}

pub fn audio_sample_batch_callback(_data: *const i16, _frames: usize) -> usize {
    println!("audio_sample_batch_callback -> {_frames}");
    0
}

pub fn video_refresh_callback(
    _data: *const ::std::os::raw::c_void,
    _width: u32,
    _height: u32,
    _pitch: usize,
) {
    println!("video_refresh_callback -> width:{_width} height:{_height} pitch:{_pitch}")
}

#[doc = "cria uma nova instancia de retro_ab_av. todas as instancias so podem ser criadas dentro da thread principal!"]
pub fn get_instance() -> RetroAVInstance {
    window::create_instance()
}

#[doc = "inicializa um instancia criada com 'get_instance'. automaticamente um janela é criada em novo perfio de audio é adicionado"]
pub fn init(av_instance: RetroAVInstance, av_info: Arc<AvInfo>) -> RetroAvCtx {
    window::create(av_instance, av_info)
}

pub fn de_init(win_ctx: &mut RetroAvCtx) {
    win_ctx.hide();
}
