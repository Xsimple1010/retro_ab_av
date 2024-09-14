use crate::retro_gl::window::GlWIndow;
use retro_ab::{
    core::AvInfo,
    retro_sys::retro_hw_context_type::{RETRO_HW_CONTEXT_NONE, RETRO_HW_CONTEXT_OPENGL_CORE},
};
use sdl2::Sdl;
use std::{
    ffi::{c_uint, c_void},
    ptr::{addr_of, addr_of_mut, null},
    sync::Arc,
};

static mut WINDOW_CTX: Option<Box<dyn RetroVideoAPi>> = None;

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

pub fn get_proc_address(procname: &str) -> *const () {
    unsafe {
        if let Some(window) = &*addr_of!(WINDOW_CTX) {
            window.get_proc_address(procname)
        } else {
            null()
        }
    }
}

pub trait RetroVideoAPi {
    fn get_window_id(&self) -> u32;

    fn draw_new_frame(&mut self, texture: &RawTextureData);

    fn resize(&mut self, new_size: (u32, u32));

    fn get_proc_address(&self, procname: &str) -> *const ();
}

pub struct RetroVideo;

impl Drop for RetroVideo {
    fn drop(&mut self) {
        unsafe {
            WINDOW_CTX.take();
        }
    }
}

impl RetroVideo {
    pub fn new(sdl: &Sdl, av_info: &Arc<AvInfo>) -> Result<Self, String> {
        match &av_info.video.graphic_api.context_type {
            RETRO_HW_CONTEXT_OPENGL_CORE | RETRO_HW_CONTEXT_NONE => {
                unsafe { WINDOW_CTX = Some(Box::new(GlWIndow::new(sdl, av_info)?)) }

                return Ok(Self);
            }
            // RETRO_HW_CONTEXT_VULKAN => {}
            _ => Err("suporte para a api selecionada não está disponível".to_owned()),
        }
    }

    pub fn draw_new_frame(&mut self) {
        unsafe {
            if let Some(window) = &mut *addr_of_mut!(WINDOW_CTX) {
                window.draw_new_frame(&*addr_of!(RAW_TEX_POINTER))
            }
        }
    }

    pub fn get_window_id(&self) -> u32 {
        unsafe {
            if let Some(window) = &mut *addr_of_mut!(WINDOW_CTX) {
                window.get_window_id()
            } else {
                0
            }
        }
    }

    pub fn resize(&mut self, new_size: (u32, u32)) {
        unsafe {
            if let Some(window) = &mut *addr_of_mut!(WINDOW_CTX) {
                window.resize(new_size)
            }
        }
    }
}
