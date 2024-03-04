use retro_ab::core::AvInfo;
use std::sync::Arc;
use winit::error::EventLoopError;
use winit::event_loop::EventLoop;

use crate::audios::{self, RetroAudio};
use crate::video::{self, RetroVideo};

pub struct RetroAvCtx {
    pub video: RetroVideo,
    pub _audio: RetroAudio,
    info: Arc<AvInfo>,
}

impl Drop for RetroAvCtx {
    fn drop(&mut self) {}
}

impl RetroAvCtx {
    pub fn get_new_frame(&mut self) {
        self._audio.resume_new_frame(&self.info);
        self.video.draw_new_frame(&self.info);
        // println!("Audio");
    }
}

pub fn create(av_info: Arc<AvInfo>) -> Result<(RetroAvCtx, EventLoop<()>), EventLoopError> {
    let (video, event_loop) = video::init(&av_info)?;
    let _audio = audios::init();

    Ok((
        RetroAvCtx {
            video,
            _audio,
            info: av_info,
        },
        event_loop,
    ))
}
