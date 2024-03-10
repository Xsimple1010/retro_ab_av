use retro_ab::core::AvInfo;
use sdl2::{EventPump, Sdl};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::audios::{self, RetroAudio};
use crate::video::{self, RetroVideo};

pub struct RetroAvCtx {
    pub video: RetroVideo,
    pub _audio: RetroAudio,
    av_info: Arc<AvInfo>,
    _sdl: Sdl,
}

impl Drop for RetroAvCtx {
    fn drop(&mut self) {}
}

impl RetroAvCtx {
    #[doc = "cria uma nova instancia de RetroAvCtx. todas as instancias so podem ser criadas dentro da thread principal!"]
    pub fn new(av_info: Arc<AvInfo>) -> Result<(RetroAvCtx, EventPump), String> {
        let _sdl = sdl2::init()?;

        let event_pump = _sdl.event_pump()?;

        let video = video::init(&_sdl, &av_info)?;
        let _audio = audios::init(&_sdl, &av_info)?;

        Ok((
            RetroAvCtx {
                video,
                _audio,
                _sdl,
                av_info: av_info.clone(),
            },
            event_pump,
        ))
    }

    pub fn get_new_frame(&mut self) -> Result<(), String> {
        let start = Instant::now();

        self._audio.resume_new_frame()?;
        self.video.draw_new_frame();

        //isso trava a taxa de quandros pelo o que foi fornecido pelo núcleo
        let end = Instant::now() - start;
        let fps_delay = (915.0 / *self.av_info.timing.fps.lock().unwrap() as f32
            - end.as_millis() as f32)
            * 1_000_000.0 as f32;

        if end.as_nanos() as f32 > fps_delay {
            std::thread::sleep(Duration::from_millis(16));
            return Ok(());
        }

        std::thread::sleep(Duration::from_nanos(
            fps_delay as u64 - end.as_nanos() as u64,
        ));

        Ok(())
    }
}
