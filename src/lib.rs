extern crate glutin;
extern crate glutin_winit;
extern crate raw_window_handle;
extern crate retro_ab;
extern crate rodio;
extern crate winit;

mod audios;
pub mod context;
pub mod retro_gl;
pub mod video;
pub use audios::{audio_sample_batch_callback, audio_sample_callback};
pub use video::video_refresh_callback;

pub use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{EventLoop, EventLoopBuilder},
    keyboard::{Key, NamedKey},
    platform::{pump_events::EventLoopExtPumpEvents, windows::EventLoopBuilderExtWindows},
};
