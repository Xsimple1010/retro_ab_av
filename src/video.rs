use crate::retro_vk::vk_instance::RetroVk;
use retro_ab::core::AvInfo;
use std::{ffi::c_uint, os::raw::c_void, ptr::null, sync::Arc};

pub struct RawTextureData {
    pub data: *const c_void,
    pub width: c_uint,
    pub height: c_uint,
    pub pitch: usize,
}

static mut RAW_TEX_POINTER: RawTextureData = RawTextureData {
    data: null(),
    pitch: 0,
    height: 0,
    width: 0,
};

pub fn video_refresh_callback(data: *const c_void, width: c_uint, height: c_uint, pitch: usize) {
    unsafe {
        RAW_TEX_POINTER.data = data;
        RAW_TEX_POINTER.height = height;
        RAW_TEX_POINTER.width = width;
        RAW_TEX_POINTER.pitch = pitch;
    }
}

pub struct RetroVideo {
    _av_info: Arc<AvInfo>,
    vulkan: RetroVk,
}

impl RetroVideo {
    pub fn new(av_info: &Arc<AvInfo>) -> Result<RetroVideo, String> {
        Ok(RetroVideo {
            _av_info: av_info.clone(),
            vulkan: RetroVk::new(),
        })
    }
    pub fn draw_new_frame(&mut self) {}
}
