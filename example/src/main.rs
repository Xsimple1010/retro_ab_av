use retro_ab::{
    core::{self, RetroContext, RetroEnvCallbacks},
    retro_sys::retro_rumble_effect,
    test_tools,
};
use retro_ab_av::{
    audio_sample_batch_callback, audio_sample_callback, context::RetroAvCtx, video_refresh_callback,
};
use std::sync::Arc;

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

//essas callbacks nao sao relevantes para esse projeto!
fn input_poll_callback() {}
fn input_state_callback(_port: i16, _device: i16, _index: i16, _id: i16) -> i16 {
    0
}

fn main() {
    let core_ctx = create_core_ctx();

    RetroAvCtx::new(core_ctx.core.av_info.clone());

    core::run(&core_ctx).expect("msg");

    let _ = core::de_init(core_ctx);
}
