use retro_ab::{
    args_manager::RetroArgs, core::RetroEnvCallbacks, retro_ab::RetroAB,
    retro_sys::retro_rumble_effect, test_tools,
};
use retro_ab_av::{
    audio_sample_batch_callback, audio_sample_callback, context::RetroAvCtx, get_proc_address,
    video_refresh_callback, Event, Keycode,
};
use std::{env, sync::Arc};

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

fn te() {
    println!("teste");
}

fn create_core_ctx() -> RetroAB {
    let args = RetroArgs::new().unwrap();

    let core_ctx = RetroAB::new(
        &args.core,
        test_tools::paths::get_paths().unwrap(),
        RetroEnvCallbacks {
            audio_sample_batch_callback,
            audio_sample_callback,
            input_poll_callback,
            input_state_callback,
            video_refresh_callback,
            rumble_callback,
            context_destroy: te,
            context_reset: te,
            get_proc_address,
        },
        retro_ab::retro_sys::retro_hw_context_type::RETRO_HW_CONTEXT_OPENGL_CORE,
    )
    .unwrap();

    let _ = core_ctx.core().init();

    core_ctx.core().load_game(&args.core).unwrap();

    core_ctx
}

fn create_new_game_window() -> Result<(), &'static str> {
    let core_ctx = create_core_ctx();

    let (mut av_ctx, mut event_pump) =
        RetroAvCtx::new(Arc::clone(&core_ctx.core().av_info)).unwrap();

    'running: loop {
        if av_ctx.sync() {
            core_ctx.core().run().expect("msg");
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

    Ok(())
}

fn main() -> Result<(), &'static str> {
    for _ in 0..1 {
        create_new_game_window()?;
    }

    Ok(())
}
