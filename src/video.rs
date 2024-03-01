use std::sync::Arc;

use retro_ab::core::AvInfo;

pub fn video_refresh_callback(
    data: *const ::std::os::raw::c_void,
    width: ::std::os::raw::c_uint,
    height: ::std::os::raw::c_uint,
    pitch: usize,
) {
}

pub struct RetroVideo {}

pub fn init(av_info: &Arc<AvInfo>) -> RetroVideo {
    RetroVideo {}
}
