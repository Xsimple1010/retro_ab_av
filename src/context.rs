use std::sync::Arc;

use retro_ab::core::AvInfo;
use winit::{error::EventLoopError, event_loop::EventLoop};

use crate::video::{self, RetroVideo};

pub struct RetroAvCtx {
    pub video: RetroVideo,
    pub info: Arc<AvInfo>,
}

impl Drop for RetroAvCtx {
    fn drop(&mut self) {}
}

impl RetroAvCtx {}

pub fn create(av_info: Arc<AvInfo>) -> Result<(RetroAvCtx, EventLoop<()>), EventLoopError> {
    let (retro_video, event_loop) = video::init(&av_info)?;

    Ok((
        RetroAvCtx {
            info: av_info,
            video: retro_video,
        },
        event_loop,
    ))
}
