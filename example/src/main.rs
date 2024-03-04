use retro_ab::{
    core::{self, RetroEnvCallbacks},
    test_tools,
};
use retro_ab_av::{
    audio_sample_batch_callback, audio_sample_callback, event, keyboard, video_refresh_callback,
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

    let (mut av_ctx, event_loop) =
        retro_ab_av::init(Arc::clone(&core_ctx.core.av_info)).expect("msg");

    let result = event_loop.run(|event, window_target| {
        match event {
            event::Event::Resumed => {}
            event::Event::Suspended => {}
            event::Event::AboutToWait => {
                av_ctx.video.window.request_redraw();
                retro_ab::core::run(&core_ctx).expect("falha ao requisitar um novo frame");
            }
            event::Event::WindowEvent { event, .. } => match event {
                event::WindowEvent::Resized(_new_size) => {
                    // av_ctx.video.resize(new_size.into());
                }
                event::WindowEvent::RedrawRequested => {
                    av_ctx.get_new_frame();
                    // av_ctx.video.draw_new_frame();
                }
                event::WindowEvent::CloseRequested
                | event::WindowEvent::KeyboardInput {
                    event:
                        event::KeyEvent {
                            state: event::ElementState::Pressed,
                            logical_key: keyboard::Key::Named(keyboard::NamedKey::Escape),
                            ..
                        },
                    ..
                } => window_target.exit(),
                _ev => {}
            },
            _ => (),
        };
    });
    result.unwrap();

    let _ = core::de_init(core_ctx);
    // retro_ab_av::de_init(av_ctx);
}
