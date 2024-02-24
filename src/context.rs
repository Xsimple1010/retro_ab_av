use std::{cell::RefCell, sync::Arc};

use retro_ab::core::AvInfo;
use sdl2::{
    video::{GLContext, GLProfile, Window},
    EventPump, Sdl, VideoSubsystem,
};

pub struct RetroAVInstance {
    pub sdl: Sdl,
}
pub struct RetroAvCtx {
    instance: RetroAVInstance,
    video: RetroVideo,
    pub info: Arc<AvInfo>,
}

pub struct RetroVideo {
    _v_subsystem: VideoSubsystem,
    _gl_ctx: GLContext,
    win: RefCell<Window>,
}

impl RetroAvCtx {
    pub fn get_event(&self) -> EventPump {
        self.instance.sdl.event_pump().unwrap()
    }

    pub fn swap(&self) {
        self.video.win.borrow_mut().gl_swap_window();
    }

    pub fn hide(&self) {
        self.video.win.borrow_mut().hide();
    }

    pub fn show(&self) {
        self.video.win.borrow_mut().show();
    }
}

fn init_video_subsystem(instance: &RetroAVInstance, av_info: &Arc<AvInfo>) -> RetroVideo {
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

    RetroVideo {
        _v_subsystem: video_subsystem,
        _gl_ctx,
        win: RefCell::new(window),
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
