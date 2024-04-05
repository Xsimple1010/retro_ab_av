use retro_ab::{
    core::{self, RetroContext, RetroEnvCallbacks},
    erro_handle::ErroHandle,
    retro_sys::retro_rumble_effect,
    test_tools,
};
use retro_ab_av::{
    audio_sample_batch_callback, audio_sample_callback,
    context::{RetroAvCtx, RetroAvEvents},
    video_refresh_callback, Event, Key, KeyEvent, NamedKey, WindowEvent,
};
use std::sync::Arc;

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

fn create_core_ctx() -> Arc<RetroContext> {
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

    core_ctx
}

fn game_loop(av_events: &mut RetroAvEvents) {
    let core_ctx = create_core_ctx();
    let mut av_ctx = RetroAvCtx::new(core_ctx.core.av_info.clone(), av_events);

    let mut running = true;

    while running {
        core::run(&core_ctx).unwrap();
        av_ctx.request_redraw();

        av_events.pump(|event, window_target| match event {
            Event::Resumed => {
                av_ctx.resume(window_target);
            }
            Event::Suspended => {
                av_ctx.suspended();
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            logical_key: Key::Named(NamedKey::Escape),
                            ..
                        },
                    ..
                } => {
                    av_ctx.close_window(window_target);
                    running = false;
                }
                WindowEvent::RedrawRequested => {
                    let _ = av_ctx.get_new_frame();
                }
                _ => (),
            },
            _ => (),
        });
    }

    let _ = core::de_init(core_ctx);
}

fn main() -> Result<(), ErroHandle> {
    let mut av_events = RetroAvEvents::new()?;

    for _ in 0..2 {
        game_loop(&mut av_events);
    }
    Ok(())
}
