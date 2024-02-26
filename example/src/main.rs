use std::sync::Arc;

use retro_ab::{
    core::{self, RetroEnvCallbacks},
    test_tools,
};
use retro_ab_av::{
    audio_sample_batch_callback, audio_sample_callback, video_refresh_callback, Event, Keycode,
};

fn input_poll_callback() {}
fn input_state_callback(_port: i16, _device: i16, _index: i16, _id: i16) -> i16 {
    0
}

fn main() {
    let ctx = core::load(
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
    .expect("nao foi possível carrega o núcleo");

    let mut state = false;

    match core::init(&ctx) {
        Ok(..) => {
            match core::load_game(&ctx, "C:/WFL/roms/teste.sfc") {
                Ok(suss) => state = suss,
                Err(e) => {
                    println!("[{:?}]: message -> {:?}", e.level, e.message)
                }
            };
        }
        Err(e) => println!("[{:?}]: message -> {:?}", e.level, e.message),
    };

    if !state {
        return;
    }

    //isso deve ser inicializado somente na thread principal!
    let av_instance = retro_ab_av::get_instance();

    //se voce estive criando uma interface provavelmente vai querer
    //fazer com que a renderização seja feita em uma thread separada,
    //por isso retro_ab_av::init pegar para si a pose de av_instance.
    //então no multithreading voce dever garantir que retro_ab_av::init sera chamado
    //dentro a thread em que voce for fazer a renderização.
    let av_ctx = retro_ab_av::init(av_instance, Arc::clone(&ctx.core.av_info));
    let mut event_pump = av_ctx.get_event();

    let mut frame = 0;

    'running: loop {
        if frame == 500 {
            break 'running;
        }

        frame = frame + 1;

        //As callbacks nao tem
        retro_ab_av::update_extras(&av_ctx);

        av_ctx.swap();

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

        match core::run(&ctx) {
            Ok(..) => {}
            Err(e) => {
                println!("[{:?}]: message -> {:?}", e.level, e.message)
            }
        }
    }

    av_ctx.hide();
    let _ = core::de_init(ctx);
    retro_ab_av::de_init(av_ctx);
}
