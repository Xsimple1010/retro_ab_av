use retro_ab::core::AvInfo;
use rodio::{buffer::SamplesBuffer, OutputStream, OutputStreamHandle, Sink};
use std::{
    ptr::{null, slice_from_raw_parts},
    sync::Arc,
};

struct AudioNewFrame {
    data: *const i16,
    frames: usize,
    channel: u16,
}

static mut NEW_FRAME: AudioNewFrame = AudioNewFrame {
    data: null(),
    frames: 0,
    channel: 2,
};

pub fn audio_sample_batch_callback(data: *const i16, frames: usize) -> usize {
    unsafe {
        NEW_FRAME = AudioNewFrame {
            data,
            frames,
            channel: 2,
        };
    }

    frames
}

pub fn audio_sample_callback(left: i16, right: i16) {
    println!("audio_sample_callback");

    unsafe {
        NEW_FRAME = AudioNewFrame {
            data: [left, right].as_ptr(),
            frames: 1,
            channel: 1,
        };
    }
}

pub struct RetroAudio {
    _stream_handle: OutputStreamHandle,
    _stream: OutputStream,
    av_info: Arc<AvInfo>,
    sink: Sink,
}

impl RetroAudio {
    pub fn init(av_info: &Arc<AvInfo>) -> Result<Self, String> {
        let (stream, stream_handle) = OutputStream::try_default().expect("msg");

        let sink: Sink = Sink::try_new(&stream_handle).expect("msg");

        Ok(Self {
            _stream: stream,
            _stream_handle: stream_handle,
            av_info: av_info.clone(),
            sink,
        })
    }

    pub fn resume_new_frame(&mut self) {
        if let Ok(sample_rate) = self.av_info.timing.sample_rate.try_lock() {
            let data = unsafe { &*slice_from_raw_parts(NEW_FRAME.data, NEW_FRAME.frames * 2) };

            let channel = unsafe { NEW_FRAME.channel };

            let sample_buffer = SamplesBuffer::new(channel, *sample_rate as u32, data);

            self.sink.append(sample_buffer);
        }
    }
}
