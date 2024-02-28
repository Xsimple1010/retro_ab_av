extern crate sdl2;
mod audio;
mod binding;
mod context;
mod video;

use context::{RetroAVInstance, RetroAvCtx};
use retro_ab::core::AvInfo;
use std::os::raw::c_void;
use std::sync::Arc;

use binding::binding_handle_retro_cb::*;

pub use audio::{audio_sample_batch_callback, audio_sample_callback};
pub use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;
pub use sdl2::EventPump;
pub use video::video_refresh_callback;

pub fn update_extras(retro_av_ctx: &Arc<RetroAvCtx>) {
    unsafe {
        set_video_extra_data(Arc::into_raw(retro_av_ctx.clone()) as *mut c_void);
        set_audio_extra_data(Arc::into_raw(retro_av_ctx.clone()) as *mut c_void);
    }
}

#[doc = "cria uma nova instancia de retro_ab_av. todas as instancias so podem ser criadas dentro da thread principal!"]
pub fn get_instance() -> RetroAVInstance {
    context::create_instance()
}

#[doc = "inicializa um instancia criada com 'get_instance'. automaticamente uma janela é criada em novo perfio de audio é adicionado"]
pub fn init(av_instance: RetroAVInstance, av_info: Arc<AvInfo>) -> Arc<RetroAvCtx> {
    unsafe {
        set_rust_video_refresh(Some(video::rust_video_refresh_callback));
        set_rust_audio_sample(Some(audio::rust_audio_sample));
        set_rust_audio_sample_batch(Some(audio::rust_audio_sample_batch_callback));
    }
    let ctx = context::create(av_instance, av_info);
    Arc::new(ctx)
}

#[doc = "eliminar o contexto atual, voce dever chamar isso sempre que nao for mais usar um contexto!"]
pub fn de_init(_av_ctx: Arc<RetroAvCtx>) {
    println!("{:?}", Arc::strong_count(&_av_ctx));

    unsafe {
        de_init_all_callbacks();
        let v_ptr = get_video_extra_data();

        if !v_ptr.is_null() {
            Arc::from_raw(v_ptr);
        }

        let a_ptr = get_audio_extra_data();

        if !a_ptr.is_null() {
            Arc::from_raw(a_ptr);
        }
    }

    println!("{:?}", Arc::strong_count(&_av_ctx));
}
