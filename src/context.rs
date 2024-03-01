use std::sync::Arc;

use retro_ab::core::AvInfo;

use crate::video::{self, RetroVideo};

pub struct RetroAvCtx {
    video: RetroVideo,
    pub info: Arc<AvInfo>,
}

impl Drop for RetroAvCtx {
    fn drop(&mut self) {
        unsafe {}
    }
}

impl RetroAvCtx {}

pub fn create(av_info: Arc<AvInfo>) -> RetroAvCtx {
    let video = video::init(&av_info);

    RetroAvCtx {
        info: av_info,
        video,
    }
}
