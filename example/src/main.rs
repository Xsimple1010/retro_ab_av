use std::{env, sync::Arc};

use retro_ab::{
    core::{self, RetroContext, RetroEnvCallbacks},
    retro_sys::retro_rumble_effect,
    test_tools,
};

use retro_ab_av::{
    audio_sample_batch_callback, audio_sample_callback, context::RetroAvCtx,
    video_refresh_callback, Event, Keycode,
};

//essas callbacks nao sao relevantes para esse projeto!
fn input_poll_callback() {}
fn input_state_callback(_port: i16, _device: i16, _index: i16, _id: i16) -> i16 {
    0
}
fn rumble_callback(
    _port: std::os::raw::c_uint,
    _effect: retro_rumble_effect,
    _strength: u16,
) -> bool {
    true
}

fn create_core_ctx() -> Result<Arc<RetroContext>, &'static str> {
    let values = retro_ab::args_manager::get_values(env::args().collect());

    match values.get_key_value("core") {
        Some((_, path)) => {
            println!("{:?}", path);

            let core_ctx = core::load(
                path,
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

            if let Some((_, rom)) = values.get_key_value("rom") {
                println!("{:?}", rom);
                core::init(&core_ctx).expect("Erro ao tentar inicializar o contexto");
                core::load_game(&core_ctx, rom).expect("Erro ao tentar carrega a rom");
            }

            return Ok(core_ctx);
        }
        _ => {}
    }

    Err("Erro ao tentar criar RetroContext: ")
}

fn create_new_game_window() -> Result<(), &'static str> {
    let core_ctx = create_core_ctx()?;

    let (mut av_ctx, mut event_pump) =
        RetroAvCtx::new(Arc::clone(&core_ctx.core.av_info)).expect("erro");

    'running: loop {

        if av_ctx.sync() {
            core::run(&core_ctx).expect("msg");
            av_ctx.get_new_frame();
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {}
                Event::Window {
                    timestamp: _,
                    window_id: _,
                    win_event,
                } => match win_event {
                    retro_ab_av::WindowEvent::Close => break 'running,
                    _ => {}
                },
                _ => {}
            }
        }
    }

    let _ = core::de_init(core_ctx);

    Ok(())
}

fn main() -> Result<(), &'static str> {
    for _ in 0..2 {
        create_new_game_window()?;
    }

    Ok(())
}
