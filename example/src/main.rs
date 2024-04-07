use retro_ab::{
    core::{self, RetroContext, RetroEnvCallbacks},
    retro_sys::retro_rumble_effect,
    test_tools,
};

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

use retro_ab_av::{
    audio_sample_batch_callback, audio_sample_callback, context::RetroAvCtx,
    video_refresh_callback, Event, Keycode,
};
use std::sync::Arc;

//essas callbacks nao sao relevantes para esse projeto!
fn input_poll_callback() {}
fn input_state_callback(_port: i16, _device: i16, _index: i16, _id: i16) -> i16 {
    0
}

fn main() {
    let core_ctx = create_core_ctx();

    let (mut av_ctx, mut event_pump) =
        RetroAvCtx::new(Arc::clone(&core_ctx.core.av_info)).expect("erro");

    'running: loop {
        core::run(&core_ctx).expect("msg");
        av_ctx.get_new_frame().expect("");

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {}
                Event::Window {
                    timestamp: _,
                    window_id,
                    win_event,
                } => {
                    if window_id == av_ctx.video.get_window_id() {
                        match win_event {
                            retro_ab_av::WindowEvent::Close => {
                                println!("janela destroida");
                            }
                            _ => break 'running,
                        }
                    }
                }
                _ => {}
            }
        }
    }

    let _ = core::de_init(core_ctx);
}
