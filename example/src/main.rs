use std::{env, sync::Arc};
use retro_ab::{
    core::RetroEnvCallbacks,
    retro_sys::retro_rumble_effect,
    test_tools,
};
use retro_ab::retro_context::RetroContext;
use retro_ab_av::{
    context::RetroAvCtx
    , Event, Keycode,
    audio_sample_batch_callback, audio_sample_callback, video_refresh_callback,
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
            let core_ctx = RetroContext::new(
                path,
                test_tools::paths::get_paths().unwrap(),
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
                core_ctx.core.load_game(rom).expect("Erro ao tentar carrega a rom");
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
            core_ctx.core.run().expect("msg");
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

    let _ = core_ctx.delete();

    Ok(())
}

fn main() -> Result<(), &'static str> {
    for _ in 0..1 {
        create_new_game_window()?;
    }

    Ok(())
}
