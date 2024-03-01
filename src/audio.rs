use std::sync::Arc;

use retro_ab::core::AvInfo;

pub fn audio_sample_batch_callback(_data: *const i16, _frames: usize) -> usize {
    0
}

pub fn audio_sample_callback(_left: i16, _right: i16) {}

pub fn init(_av_info: &Arc<AvInfo>) {}
