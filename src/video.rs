use std::{cell::RefCell, sync::Arc};

use retro_ab::core::AvInfo;
use sdl2::{
    video::{GLContext, GLProfile, Window},
    VideoSubsystem,
};

use crate::{
    binding,
    context::{RetroAVInstance, RetroAvCtx},
};

pub fn video_refresh_callback(
    data: *const ::std::os::raw::c_void,
    width: ::std::os::raw::c_uint,
    height: ::std::os::raw::c_uint,
    pitch: usize,
) {
    unsafe { binding::binding_handle_retro_cb::core_video_refresh(data, width, height, pitch) }
}

pub unsafe extern "C" fn rust_video_refresh_callback(
    extra_data: *mut ::std::os::raw::c_void,
    _data: *const ::std::os::raw::c_void,
    _width: ::std::os::raw::c_uint,
    _height: ::std::os::raw::c_uint,
    _pitch: usize,
) {
    if !extra_data.is_null() {
        let v: Arc<RetroAvCtx> = Arc::from_raw(extra_data as *const RetroAvCtx);
        println!(
            "rust_video_refresh_callback: pixel ->  {:?}",
            v.info.video.pixel_format.try_lock().unwrap()
        );
    }
}

pub struct RetroVideo {
    pub _v_subsystem: VideoSubsystem,
    pub _gl_ctx: GLContext,
    pub win: RefCell<Window>,
}

pub fn init(instance: &RetroAVInstance, av_info: &Arc<AvInfo>) -> RetroVideo {
    let video_subsystem: VideoSubsystem = instance.sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window(
            "title",
            *av_info.video.geometry.base_width.lock().unwrap(),
            *av_info.video.geometry.base_height.lock().unwrap(),
        )
        .opengl()
        .resizable()
        .position_centered()
        .build()
        .unwrap();
    let _gl_ctx = window.gl_create_context().unwrap();

    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(gl_attr.context_version(), (3, 3));

    RetroVideo {
        _v_subsystem: video_subsystem,
        _gl_ctx,
        win: RefCell::new(window),
    }
}
