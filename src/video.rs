use crate::retro_gl::{
    gl::{get_gl_context, gl_config_picker},
    render::Render,
    RawTextureData,
};
use glutin::config::{Config, ConfigTemplateBuilder};
use glutin::context::{NotCurrentContext, PossiblyCurrentContext};
use glutin::display::{Display, GetGlDisplay};
use glutin::prelude::*;
use glutin::surface::{Surface, WindowSurface};
use glutin_winit::{self, DisplayBuilder, GlWindow};
use raw_window_handle::HasRawWindowHandle;
use retro_ab::core::AvInfo;
use std::error::Error;
use std::ffi::c_uint;
use std::os::raw::c_void;
use std::ptr::null;
use std::sync::Arc;
use winit::dpi::PhysicalSize;
use winit::event_loop::{EventLoop, EventLoopBuilder, EventLoopWindowTarget};
use winit::window::{Window, WindowBuilder};

static mut RAW_TEX_POINTER: RawTextureData = RawTextureData {
    data: null(),
    pitch: 0,
    height: 0,
    width: 0,
};

pub fn video_refresh_callback(data: *const c_void, width: c_uint, height: c_uint, pitch: usize) {
    unsafe {
        RAW_TEX_POINTER.data = data;
        RAW_TEX_POINTER.height = height;
        RAW_TEX_POINTER.width = width;
        RAW_TEX_POINTER.pitch = pitch;
    }
}

pub struct RetroVideo {
    pub state: Option<(PossiblyCurrentContext, Surface<WindowSurface>, Window)>,
    pub las_window_size: PhysicalSize<u32>,
    window: Option<Window>,
    not_current_gl_context: Option<NotCurrentContext>,
    av_info: Arc<AvInfo>,
    render: Option<Render>,
    gl_config: Config,
    gl_display: Display,
}

impl RetroVideo {
    pub fn draw_new_frame(&mut self) {
        unsafe {
            if let Some(render) = &mut self.render {
                render.draw_new_frame(
                    &RAW_TEX_POINTER,
                    &self.av_info.video.geometry,
                    self.las_window_size.width as i32,
                    self.las_window_size.height as i32,
                );
            }
        }
    }

    pub fn resume(&mut self, window_target: &EventLoopWindowTarget<()>) {
        #[cfg(android_platform)]
        println!("Android window available");

        let window = self.window.take().unwrap_or_else(|| {
            let window_builder =
                WindowBuilder::new().with_title("Retro_ab_av (press Escape to exit)");
            glutin_winit::finalize_window(window_target, window_builder, &self.gl_config).unwrap()
        });

        let attrs = window.build_surface_attributes(Default::default());
        let gl_surface = unsafe {
            self.gl_config
                .display()
                .create_window_surface(&self.gl_config, &attrs)
                .unwrap()
        };

        let gl_context: PossiblyCurrentContext = self
            .not_current_gl_context
            .take()
            .unwrap()
            .make_current(&gl_surface)
            .unwrap();

        self.render
            .replace(Render::new(&self.av_info, &self.gl_display).unwrap());

        assert!(self
            .state
            .replace((gl_context, gl_surface, window))
            .is_none());
    }

    pub fn suspended(&mut self) {
        println!("Android window removed");

        let (gl_context, ..) = self.state.take().unwrap();

        assert!(self
            .not_current_gl_context
            .replace(gl_context.make_not_current().unwrap())
            .is_none());
    }
}

pub fn init(av_info: Arc<AvInfo>) -> Result<(RetroVideo, EventLoop<()>), Box<dyn Error>> {
    let event_loop = EventLoopBuilder::new().build().unwrap();
    // Only Windows requires the window to be present before creating the display.
    // Other platforms don't really need one.
    //
    // XXX if you don't care about running on Android or so you can safely remove
    // this condition and always pass the window builder.
    let window_builder = cfg!(wgl_backend).then(|| {
        WindowBuilder::new()
            .with_title("Glutin triangle gradient example (press Escape to exit)")
            .with_maximized(true)
    });

    // The template will match only the configurations supporting rendering
    // to windows.
    //
    // XXX We force transparency only on macOS, given that EGL on X11 doesn't
    // have it, but we still want to show window. The macOS situation is like
    // that, because we can query only one config at a time on it, but all
    // normal platforms will return multiple configs, so we can find the config
    // with transparency ourselves inside the `reduce`.
    let template = ConfigTemplateBuilder::new().with_alpha_size(8);

    let display_builder = DisplayBuilder::new().with_window_builder(window_builder);

    let (window, gl_config) = display_builder.build(&event_loop, template, gl_config_picker)?;

    println!("Picked a config with {} samples", gl_config.num_samples());

    let raw_window_handle = window.as_ref().map(|window| window.raw_window_handle());

    // XXX The display could be obtained from any object created by it, so we can
    // query it from the config.
    let gl_display = gl_config.display();

    let not_current_gl_context = get_gl_context(raw_window_handle, &gl_display, &gl_config);

    Ok((
        RetroVideo {
            av_info,
            gl_config,
            not_current_gl_context,
            window,
            gl_display,
            render: None,
            state: None,
            las_window_size: PhysicalSize {
                width: 200,
                height: 200,
            },
        },
        event_loop,
    ))
}
