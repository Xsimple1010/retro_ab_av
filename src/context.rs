use retro_ab::core::AvInfo;
use retro_ab::erro_handle::ErroHandle;
use retro_ab::retro_sys::retro_log_level;
use std::sync::Arc;
use std::time::{Duration, Instant};
use winit::event::Event;
use winit::event_loop::{EventLoop, EventLoopBuilder, EventLoopWindowTarget};
use winit::platform::pump_events::EventLoopExtPumpEvents;
use winit::platform::windows::EventLoopBuilderExtWindows;

use crate::audios::{self, RetroAudio};
use crate::video::RetroVideo;

pub struct RetroAvEvents {
    pub event_loop: EventLoop<()>,
}

impl RetroAvEvents {
    pub fn new() -> Result<RetroAvEvents, ErroHandle> {
        match EventLoopBuilder::new().with_any_thread(true).build() {
            Ok(event_loop) => Ok(RetroAvEvents { event_loop }),
            Err(e) => Err(ErroHandle {
                level: retro_log_level::RETRO_LOG_ERROR,
                message: e.to_string(),
            }),
        }
    }

    pub fn pump<T>(&mut self, event_handler: T)
    where
        T: FnMut(Event<()>, &EventLoopWindowTarget<()>),
    {
        self.event_loop
            .pump_events(Some(Duration::ZERO), event_handler);
    }
}

pub struct RetroAvCtx {
    video: RetroVideo,
    audio: RetroAudio,
    av_info: Arc<AvInfo>,
}

impl RetroAvCtx {
    pub fn new(av_info: Arc<AvInfo>, instance: &RetroAvEvents) -> RetroAvCtx {
        Self {
            audio: audios::init(&av_info).unwrap(),
            video: RetroVideo::new(av_info.clone(), &instance.event_loop).unwrap(),
            av_info,
        }
    }

    pub fn get_new_frame(&mut self) -> Result<(), String> {
        let start = Instant::now();

        self.audio.resume_new_frame();
        self.video.draw_new_frame();

        //isso trava a taxa de quandros pelo o que foi fornecido pelo núcleo
        let end = Instant::now() - start;
        let fps_delay = (910.0 / *self.av_info.timing.fps.lock().unwrap() as f32
            - end.as_millis() as f32)
            * 1_000_000.0_f32;

        if end.as_nanos() as f32 > fps_delay {
            std::thread::sleep(Duration::from_millis(16));
            return Ok(());
        }

        std::thread::sleep(Duration::from_nanos(
            fps_delay as u64 - end.as_nanos() as u64,
        ));

        Ok(())
    }

    pub fn request_redraw(&self) {
        if let Some((_, _, window)) = &self.video.state {
            window.request_redraw();
        }
    }

    pub fn set_visibility(&self, visibility: bool) {
        if let Some((_, _, window)) = &self.video.state {
            window.set_visible(visibility);
        }
    }

    pub fn close_window(&self, window_target: &EventLoopWindowTarget<()>) {
        window_target.exit();
        self.set_visibility(false);
    }

    pub fn resume(&mut self, window_target: &EventLoopWindowTarget<()>) {
        self.video.resume(window_target)
    }

    pub fn suspended(&mut self) {
        self.video.suspended();
    }
}
