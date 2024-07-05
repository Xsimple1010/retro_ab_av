use std::sync::Arc;

use retro_ab::core::AvInfo;
use sdl2::{EventPump, Sdl};
use crate::audios::{self, RetroAudio};
use crate::sync::RetroSync;
use crate::video::{self, RetroVideo};

pub struct RetroAvCtx {
    pub video: RetroVideo,
    pub audio: RetroAudio,
    sync: RetroSync,
    av_info: Arc<AvInfo>,
    _sdl: Sdl,
}

impl RetroAvCtx {
    #[doc = "cria uma nova instancia de RetroAvCtx. todas as instancias so podem ser criadas dentro da thread principal!"]
    pub fn new(av_info: Arc<AvInfo>) -> Result<(RetroAvCtx, EventPump), String> {
        let _sdl = sdl2::init()?;

        let event_pump = _sdl.event_pump()?;

        let video = video::init(&_sdl, &av_info)?;
        let audio = audios::init(&av_info)?;

        Ok((
            RetroAvCtx {
                video,
                audio,
                _sdl,
                sync: RetroSync::default(),
                av_info: av_info.clone(),
            },
            event_pump,
        ))
    }

    pub fn get_new_frame(&mut self) {
        self.audio.resume_new_frame();
        self.video.draw_new_frame();
    }

    pub fn sync(&mut self ) -> bool {
        let fps = self.av_info.timing.fps.lock().unwrap().abs();
        self.sync.sync(fps)
    }
}
