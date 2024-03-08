use retro_ab::core::AvInfo;
use sdl2::{EventPump, Sdl};
use std::sync::Arc;
use std::time::Instant;

use crate::audios::{self, RetroAudio};
use crate::video::{self, RetroVideo};

pub struct RetroAvCtx {
    pub video: RetroVideo,
    pub _audio: RetroAudio,
    _sdl: Sdl,
}

impl Drop for RetroAvCtx {
    fn drop(&mut self) {}
}

impl RetroAvCtx {
    pub fn get_new_frame(&mut self) -> Result<(), String> {
        let inicio = Instant::now();
        self._audio.resume_new_frame()?;
        self.video.draw_new_frame();

        let duracao = Instant::now() - inicio;
        println!("Tempo de execução: {:?}", duracao);
        Ok(())
    }
}

pub fn create(av_info: Arc<AvInfo>) -> Result<(RetroAvCtx, EventPump), String> {
    let _sdl = sdl2::init()?;

    let event_pump = _sdl.event_pump()?;

    let video = video::init(&_sdl, &av_info)?;
    let _audio = audios::init(&_sdl, &av_info)?;

    Ok((
        RetroAvCtx {
            video,
            _audio,
            _sdl,
        },
        event_pump,
    ))
}
