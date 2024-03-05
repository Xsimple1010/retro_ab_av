use retro_ab::{
    core::{self, RetroEnvCallbacks},
    test_tools,
};
use retro_ab_av::{
    audio_sample_batch_callback, audio_sample_callback, video_refresh_callback, Event, Keycode,
};
use std::sync::Arc;

//essas callbacks nao sao relevantes para esse projeto!
fn input_poll_callback() {}
fn input_state_callback(_port: i16, _device: i16, _index: i16, _id: i16) -> i16 {
    0
}

fn main() {
    let core_ctx = core::load(
        "C:/Projetos/retro_ab/cores/test.dll",
        test_tools::paths::get_paths(),
        RetroEnvCallbacks {
            audio_sample_batch_callback,
            audio_sample_callback,
            input_poll_callback,
            input_state_callback,
            video_refresh_callback,
        },
    )
    .expect("Erro ao tentar criar RetroContext: ");

    core::init(&core_ctx).expect("Erro ao tentar inicializar o contexto");
    core::load_game(&core_ctx, "C:/WFL/roms/teste.sfc").expect("Erro ao tentar carrega a rom");

    let (mut av_ctx, mut event_pump) =
        retro_ab_av::init(Arc::clone(&core_ctx.core.av_info)).expect("erro");

    'running: loop {
        core::run(&core_ctx);
        av_ctx.get_new_frame();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
    }

    let _ = core::de_init(core_ctx);
    // retro_ab_av::de_init(av_ctx);
}
