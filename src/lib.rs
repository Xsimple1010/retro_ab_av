extern crate retro_ab;
extern crate rodio;
extern crate sdl2;

mod audios;
mod retro_gl;
mod video;
mod sync;

pub mod context;

pub use sdl2::event::Event;
pub use sdl2::event::WindowEvent;
pub use sdl2::keyboard::Keycode;
pub use sdl2::EventPump;

pub use audios::{audio_sample_batch_callback, audio_sample_callback};
pub use video::video_refresh_callback;
pub use sync::{RetroSync};
