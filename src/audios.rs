use retro_ab::core::AvInfo;
use sdl2::{
    audio::{AudioQueue, AudioSpecDesired},
    AudioSubsystem, Sdl,
};
use std::ptr::{null, slice_from_raw_parts};

static mut NEW_FRAME: AudioNewFrame = AudioNewFrame {
    _data: null(),
    frames: 0,
};

pub fn audio_sample_batch_callback(_data: *const i16, frames: usize) -> usize {
    unsafe {
        NEW_FRAME = AudioNewFrame { _data, frames };
    }

    frames
}

pub fn audio_sample_callback(_left: i16, _right: i16) {
    println!("audio_sample_callback")
}

struct AudioNewFrame {
    _data: *const i16,
    frames: usize,
}

pub struct RetroAudio {
    _audio: AudioSubsystem,
    _spec: AudioSpecDesired,
    device: AudioQueue<i16>,
}

impl RetroAudio {
    pub fn resume_new_frame(&mut self) -> Result<(), String> {
        unsafe {
            let data = &*slice_from_raw_parts(NEW_FRAME._data, NEW_FRAME.frames * 2);

            self.device.queue_audio(data)?;
        }

        Ok(())
    }
}

pub fn init(sdl: &Sdl, av_info: &AvInfo) -> Result<RetroAudio, String> {
    let _audio = sdl.audio()?;

    let _spec = AudioSpecDesired {
        channels: Some(2),
        freq: Some(*av_info.timing.sample_rate.lock().unwrap() as i32),
        samples: Some(4096),
    };

    let device = _audio
        .open_queue::<i16, _>(None, &_spec)
        .expect("erro ao agenda a reprodução de audio");
    device.resume();

    Ok(RetroAudio {
        _audio,
        _spec,
        device,
    })
}
