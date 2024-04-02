use retro_ab::{
    core::{self, RetroEnvCallbacks},
    retro_sys::retro_rumble_effect,
    test_tools,
};
use retro_ab_av::{
    audio_sample_batch_callback, audio_sample_callback,
    context::RetroAvCtx,
    glutin::surface::GlSurface,
    video_refresh_callback,
    winit::{
        event::{Event, KeyEvent, WindowEvent},
        keyboard::{Key, NamedKey},
    },
};

//essas callbacks nao sao relevantes para esse projeto!
fn input_poll_callback() {}
fn input_state_callback(_port: i16, _device: i16, _index: i16, _id: i16) -> i16 {
    0
}

fn rumble_callback(
    _port: ::std::os::raw::c_uint,
    _effect: retro_rumble_effect,
    _strength: u16,
) -> bool {
    true
}

fn main() {
    let core_ctx = core::load(
        "C:/WFL/cores/test.dll",
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

    let (mut ctx, event_loop) = RetroAvCtx::new(core_ctx.core.av_info.clone());

    event_loop
        .run(|event, window_target| match event {
            Event::Resumed => {
                ctx.video.resume(&window_target);
            }
            Event::Suspended => {
                ctx.video.suspended();
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => ctx.set_new_size(size),
                WindowEvent::RedrawRequested => {
                    ctx.get_new_frame().unwrap();
                }
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            logical_key: Key::Named(NamedKey::Escape),
                            ..
                        },
                    ..
                } => {
                    window_target.exit();
                }
                _ => (),
            },
            Event::AboutToWait => {
                if let Some((gl_context, gl_surface, window)) = &ctx.video.state {
                    core::run(&core_ctx).unwrap();
                    window.request_redraw();

                    gl_surface.swap_buffers(gl_context).unwrap();
                }
            }
            _ => (),
        })
        .unwrap();

    core::de_init(core_ctx.clone()).unwrap();
}
