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
    _stream_handle: OutputStreamHandle,
    _stream: OutputStream,
    av_info: Arc<AvInfo>,
    sink: Sink,
}

impl RetroAudio {
    pub fn resume_new_frame(&mut self) {
        if let Ok(sample_rate) = self.av_info.timing.sample_rate.try_lock() {
            let data = unsafe { &*slice_from_raw_parts(NEW_FRAME._data, NEW_FRAME.frames * 2) };

            let sample_buffer = SamplesBuffer::new(2, *sample_rate as u32, data);

            self.sink.append(sample_buffer);
        }
    }
}

pub fn init(av_info: &Arc<AvInfo>) -> Result<RetroAudio, String> {
    let (stream, stream_handle) = OutputStream::try_default().expect("msg");

    let sink: Sink = Sink::try_new(&stream_handle).expect("msg");

    Ok(RetroAudio {
        _stream: stream,
        _stream_handle: stream_handle,
        av_info: av_info.clone(),
        sink,
    })
}
