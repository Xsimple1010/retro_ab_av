use std::sync::Arc;

use crate::{binding, context::RetroAvCtx};

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
