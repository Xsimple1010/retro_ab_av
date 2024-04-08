use retro_ab::core::AvInfo;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::video::RetroVideo;

// use crate::audios::{self, RetroAudio};
// use crate::video::{self, RetroVideo};

pub struct RetroAvCtx {
    av_info: Arc<AvInfo>,
    video: RetroVideo,
}

impl Drop for RetroAvCtx {
    fn drop(&mut self) {}
}

impl RetroAvCtx {
    #[doc = "cria uma nova instancia de RetroAvCtx. todas as instancias so podem ser criadas dentro da thread principal!"]
    pub fn new(av_info: Arc<AvInfo>) -> Result<RetroAvCtx, String> {
        Ok(RetroAvCtx {
            video: RetroVideo::new(&av_info).unwrap(),
            av_info: av_info.clone(),
        })
    }

    pub fn get_new_frame(&mut self) -> Result<(), String> {
        let start = Instant::now();

        //isso trava a taxa de quandros pelo o que foi fornecido pelo núcleo
        let end = Instant::now() - start;
        let fps_delay = (910.0 / *self.av_info.timing.fps.lock().unwrap() as f32
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
