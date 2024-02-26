use std::sync::Arc;

use retro_ab::core::AvInfo;
use sdl2::{EventPump, Sdl};

use crate::{
    audio::{self, RetroAudio},
    de_init_all_callbacks,
    video::{self, RetroVideo},
};

pub struct RetroAVInstance {
    pub sdl: Sdl,
}
pub struct RetroAvCtx {
    instance: RetroAVInstance,
    video: RetroVideo,
    pub audio: RetroAudio,
    pub info: Arc<AvInfo>,
}

impl Drop for RetroAvCtx {
    fn drop(&mut self) {
        unsafe {
            de_init_all_callbacks();
        }
    }
}

impl RetroAvCtx {
    pub fn get_event(&self) -> EventPump {
        self.instance.sdl.event_pump().unwrap()
    }

    pub fn swap(&self) {
        self.video.win.borrow_mut().gl_swap_window();
    }

    pub fn hide(&self) {
        self.video.win.borrow_mut().hide();
    }

    pub fn show(&self) {
        self.video.win.borrow_mut().show();
    }
}

pub fn create_instance() -> RetroAVInstance {
    let sdl = sdl2::init().expect("nao foi possível inicializar a instancia");
    RetroAVInstance { sdl }
}

pub fn create(av_instance: RetroAVInstance, av_info: Arc<AvInfo>) -> RetroAvCtx {
    let video = video::init(&av_instance, &av_info);
    let audio = audio::init(&av_instance, &av_info);

    RetroAvCtx {
        instance: av_instance,
        info: av_info,
        audio,
        video,
    }
}
