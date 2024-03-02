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

    //isso deve ser inicializado somente na thread principal!
    let (mut av_ctx, event_loop) =
        retro_ab_av::init(Arc::clone(&core_ctx.core.av_info)).expect("msg");
    // let mut event_pump = av_ctx.get_event();

    let result = event_loop.run(|event, window_target| {
        match event {
            // The Resumed/Suspended events are mostly for Android compatibility since the context can get lost there at any point.
            // For convenience's sake the Resumed event is also delivered on other platforms on program startup.
            event::Event::Resumed => {}
            event::Event::Suspended => {}
            // By requesting a redraw in response to a AboutToWait event we get continuous rendering.
            // For applications that only change due to user input you could remove this handler.
            event::Event::AboutToWait => {
                // retro_ab::core::run(&core_ctx).expect("falha ao requisitar um novo frame");
                av_ctx.video.window.request_redraw();
            }
            event::Event::WindowEvent { event, .. } => match event {
                event::WindowEvent::Resized(new_size) => {
                    av_ctx.video.resize(new_size.into());
                }
                event::WindowEvent::RedrawRequested => {
                    av_ctx.video.draw_new_frame();
                }
                // Exit the event loop when requested (by closing the window for example) or when
                // pressing the Esc key.
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
                // Every other event
                _ev => {}
            },
            _ => (),
        };
    });
    result.unwrap();

    let _ = core::de_init(core_ctx);
    // retro_ab_av::de_init(av_ctx);
}
