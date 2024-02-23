use std::sync::Arc;

use retro_ab::core::AvInfo;
use sdl2::{
    video::{GLContext, GLProfile, Window},
    EventPump, Sdl, VideoSubsystem,
};

pub struct RetroAVInstance {
    sdl: Sdl,
}
pub struct RetroAvCtx {
    instance: RetroAVInstance,
    video: Video,
    info: Arc<AvInfo>,
}

pub struct Video {
    _v_subsystem: VideoSubsystem,
    _gl_ctx: GLContext,
    win: Window,
}

impl RetroAvCtx {
    pub fn get_event(&mut self) -> EventPump {
        self.instance.sdl.event_pump().unwrap()
    }

    pub fn swap(&self) {
        self.video.win.gl_swap_window();
    }

    pub fn hide(&mut self) {
        self.video.win.hide();
    }

    pub fn show(&mut self) {
        self.video.win.show();
    }
}

fn init_video_subsystem(instance: &RetroAVInstance, av_info: &Arc<AvInfo>) -> Video {
    let video_subsystem: VideoSubsystem = instance.sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window(
            "title",
            *av_info.video.geometry.base_width.lock().unwrap(),
            *av_info.video.geometry.base_height.lock().unwrap(),
        )
        .opengl()
        .resizable()
        .position_centered()
        .build()
        .unwrap();
    let _gl_ctx = window.gl_create_context().unwrap();

    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(gl_attr.context_version(), (3, 3));

    Video {
        _v_subsystem: video_subsystem,
        _gl_ctx,
        win: window,
    }
}

pub fn create_instance() -> RetroAVInstance {
    let sdl = sdl2::init().expect("nao foi possível inicializar a instancia");
    RetroAVInstance { sdl }
}

pub fn create(av_instance: RetroAVInstance, av_info: Arc<AvInfo>) -> RetroAvCtx {
    //==============================
    //=====inicializa o video======
    //==============================
    let video = init_video_subsystem(&av_instance, &av_info);

    RetroAvCtx {
        instance: av_instance,
        info: av_info,
        video,
    }
}
