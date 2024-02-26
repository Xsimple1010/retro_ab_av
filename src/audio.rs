use std::sync::Arc;

use retro_ab::core::AvInfo;
use sdl2::audio::{AudioQueue, AudioSpecDesired};

use crate::{
    binding::binding_handle_retro_cb::{core_audio_sample, core_audio_sample_batch},
    context::{RetroAVInstance, RetroAvCtx},
};

pub fn audio_sample_batch_callback(data: *const i16, frames: usize) -> usize {
    unsafe { core_audio_sample_batch(data, frames) }
}

pub fn audio_sample_callback(left: i16, right: i16) {
    unsafe { core_audio_sample(left, right) }
}

pub extern "C" fn rust_audio_sample_batch_callback(
    extra_data: *mut ::std::os::raw::c_void,
    data: *const i16,
    frames: usize,
) -> usize {
    println!("rust_audio_sample_batch_callback",);

    if !extra_data.is_null() {
        let ctx: Arc<RetroAvCtx> = unsafe { Arc::from_raw(extra_data as *const RetroAvCtx) };

        unsafe {
            let slice = std::slice::from_raw_parts(data as *const i16, frames * 2);

            ctx.audio.device.queue_audio(&slice).expect("msg");
            ctx.audio.device.resume();
        }
    }

    frames
}

pub extern "C" fn rust_audio_sample(
    _extra_data: *mut ::std::os::raw::c_void,
    left: i16,
    right: i16,
) {
    println!("rust_audio_sample -> {:?} : {:?}", left, right);
}

pub struct RetroAudio {
    pub spec: AudioSpecDesired,
    pub device: AudioQueue<i16>,
}

pub fn init(instance: &RetroAVInstance, av_info: &Arc<AvInfo>) -> RetroAudio {
    let audio_subsystem = instance
        .sdl
        .audio()
        .expect("nao foi possível inicializar o modulo de audio");

    let freq = *av_info.timing.sample_rate.lock().unwrap() as i32;

    let desired_spec = AudioSpecDesired {
        channels: Some(2),
        freq: Some(freq),
        samples: Some(4096),
    };

    let device = audio_subsystem
        .open_queue::<i16, _>(None, &desired_spec)
        .expect("nds");

    RetroAudio {
        spec: desired_spec,
        device,
    }
}
