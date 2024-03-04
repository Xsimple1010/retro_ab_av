use retro_ab::core::AvInfo;
use rodio::{buffer::SamplesBuffer, OutputStream, OutputStreamHandle, Sink};
use std::{
    ptr::{null, slice_from_raw_parts},
    sync::Arc,
};

static mut NEW_FRAME: AudioNewFrame = AudioNewFrame {
    _data: null(),
    frames: 0,
};

pub fn audio_sample_batch_callback(_data: *const i16, frames: usize) -> usize {
    unsafe {
        NEW_FRAME = AudioNewFrame {
            _data,
            frames: frames.clone(),
        };
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
    _stream_handle: OutputStreamHandle,
    _stream: OutputStream,
    sink: Sink,
}

impl RetroAudio {
    pub fn resume_new_frame(&mut self, av_info: &Arc<AvInfo>) {
        unsafe {
            let data = &*slice_from_raw_parts(NEW_FRAME._data, NEW_FRAME.frames * 2);

            let sample_buffer =
                SamplesBuffer::new(2, *av_info.timing.sample_rate.lock().unwrap() as u32, data);

            self.sink.append(sample_buffer);
        }
    }
}

pub fn init() -> RetroAudio {
    let (_stream, _stream_handle) = OutputStream::try_default().expect("msg");

    let sink: Sink = Sink::try_new(&_stream_handle).expect("msg");

    RetroAudio {
        _stream,
        _stream_handle,
        sink,
    }
}
