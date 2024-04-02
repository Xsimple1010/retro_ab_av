use std::{num::NonZeroU32, os::raw::c_void};

use retro_ab::{
    core::{self, RetroEnvCallbacks},
    retro_sys::retro_rumble_effect,
    test_tools,
};
use retro_ab_av::{
    audio_sample_batch_callback, audio_sample_callback,
    context::RetroAvCtx,
    glutin::{
        context::NotCurrentGlContext,
        display::{GetGlDisplay, GlDisplay},
        surface::{GlSurface, SwapInterval},
    },
    glutin_winit::{self, GlWindow},
    retro_gl::render::Render,
    winit::{
        event::{Event, KeyEvent, WindowEvent},
        event_loop::{EventLoop, EventLoopBuilder},
        keyboard::{Key, NamedKey},
        window::WindowBuilder,
    },
};

//essas callbacks nao sao relevantes para esse projeto!
fn input_poll_callback() {}
fn input_state_callback(_port: i16, _device: i16, _index: i16, _id: i16) -> i16 {
    0
}

fn rumble_callback(
    port: ::std::os::raw::c_uint,
    effect: retro_rumble_effect,
    strength: u16,
) -> bool {
    true
}

fn video_refresh_callback(data: *const c_void, width: u32, height: u32, pitch: usize) {}

fn tes() {
    let core_ctx = core::load(
        "C:/Projetos/retro_ab/cores/test.dll",
        test_tools::paths::get_paths(),
        RetroEnvCallbacks {
            audio_sample_batch_callback,
            audio_sample_callback,
            input_poll_callback,
            input_state_callback,
            video_refresh_callback,
            rumble_callback,
        },
    )
    .expect("Erro ao tentar criar RetroContext: ");

    core::init(&core_ctx).expect("Erro ao tentar inicializar o contexto");
    core::load_game(&core_ctx, "C:/WFL/roms/teste.sfc").expect("Erro ao tentar carrega a rom");

    let _ = RetroAvCtx::new(core_ctx.core.av_info.clone());

    // let mut state = None;
    // let mut renderer = None;

    // event_loop
    //     .run(|event, window_target| {
    //         match event {
    //             Event::Resumed => {
    //                 #[cfg(android_platform)]
    //                 println!("Android window available");

    //                 let window = av_ctx.video.window.take().unwrap_or_else(|| {
    //                     let window_builder = WindowBuilder::new()
    //                         .with_title("Glutin triangle gradient example (press Escape to exit)");
    //                     glutin_winit::finalize_window(
    //                         window_target,
    //                         window_builder,
    //                         &av_ctx.video.gl_config,
    //                     )
    //                     .unwrap()
    //                 });

    //                 let attrs = window.build_surface_attributes(Default::default());
    //                 let gl_surface = unsafe {
    //                     av_ctx
    //                         .video
    //                         .gl_config
    //                         .display()
    //                         .create_window_surface(&av_ctx.video.gl_config, &attrs)
    //                         .unwrap()
    //                 };

    //                 // Make it current.
    //                 let gl_context = av_ctx
    //                     .video
    //                     .not_current_context
    //                     .take()
    //                     .unwrap()
    //                     .make_current(&gl_surface)
    //                     .unwrap();

    //                 // The context needs to be current for the Renderer to set up shaders and
    //                 // buffers. It also performs function loading, which needs a current context on
    //                 // WGL.
    //                 renderer.get_or_insert_with(|| {
    //                     Render::new(&core_ctx.core.av_info, &av_ctx.video.gl_display)
    //                 });

    //                 // Try setting vsync.
    //                 if let Err(res) = gl_surface.set_swap_interval(
    //                     &gl_context,
    //                     SwapInterval::Wait(NonZeroU32::new(1).unwrap()),
    //                 ) {
    //                     eprintln!("Error setting vsync: {res:?}");
    //                 }

    //                 assert!(state.replace((gl_context, gl_surface, window)).is_none());
    //             }
    //             Event::Suspended => {
    //                 // // This event is only raised on Android, where the backing NativeWindow for a GL
    //                 // // Surface can appear and disappear at any moment.
    //                 // println!("Android window removed");

    //                 // // Destroy the GL Surface and un-current the GL Context before ndk-glue releases
    //                 // // the window back to the system.
    //                 // let (gl_context, ..) = state.take().unwrap();
    //                 // assert!(not_current_gl_context
    //                 //     .replace(gl_context.make_not_current().unwrap())
    //                 //     .is_none());
    //             }
    //             Event::WindowEvent { event, .. } => match event {
    //                 WindowEvent::Resized(size) => {
    //                     if size.width != 0 && size.height != 0 {
    //                         // Some platforms like EGL require resizing GL surface to update the size
    //                         // Notable platforms here are Wayland and macOS, other don't require it
    //                         // and the function is no-op, but it's wise to resize it for portability
    //                         // reasons.
    //                         // if let Some((gl_context, gl_surface, _)) = &state {
    //                         //     gl_surface.resize(
    //                         //         gl_context,
    //                         //         NonZeroU32::new(size.width).unwrap(),
    //                         //         NonZeroU32::new(size.height).unwrap(),
    //                         //     );
    //                         //     let renderer = renderer.as_ref().unwrap();
    //                         // }
    //                     }
    //                 }
    //                 WindowEvent::CloseRequested
    //                 | WindowEvent::KeyboardInput {
    //                     event:
    //                         KeyEvent {
    //                             logical_key: Key::Named(NamedKey::Escape),
    //                             ..
    //                         },
    //                     ..
    //                 } => window_target.exit(),
    //                 WindowEvent::Destroyed => {}
    //                 _ => (),
    //             },
    //             Event::AboutToWait => {
    //                 if let Some((gl_context, gl_surface, window)) = &state {
    //                     let renderer = renderer.as_ref().unwrap();
    //                     window.request_redraw();

    //                     gl_surface.swap_buffers(gl_context).unwrap();
    //                 }
    //             }

    //             _ => (),
    //         }
    //     })
    //     .unwrap();

    core::de_init(core_ctx).unwrap();

    // retro_ab_av::video::tete(EventLoopBuilder::new().build().unwrap());
}

fn main() {
    tes();
    // retro_ab_av::de_init(av_ctx);
}
