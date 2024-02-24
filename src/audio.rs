use crate::binding::binding_handle_retro_cb::{core_audio_sample, core_audio_sample_batch};

pub fn audio_sample_batch_callback(data: *const i16, frames: usize) -> usize {
    unsafe { core_audio_sample_batch(data, frames) }
}

pub fn audio_sample_callback(left: i16, right: i16) {
    unsafe { core_audio_sample(left, right) }
}

pub extern "C" fn rust_audio_sample_batch_callback(
    _extra_data: *mut ::std::os::raw::c_void,
    data: *const i16,
    frames: usize,
) -> usize {
    println!(
        "rust_audio_sample_batch_callback -> {:?} : {:?}",
        data, frames
    );
    0
}

pub extern "C" fn rust_audio_sample(
    _extra_data: *mut ::std::os::raw::c_void,
    left: i16,
    right: i16,
) {
    println!("rust_audio_sample -> {:?} : {:?}", left, right)
}
