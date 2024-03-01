mod audio;
mod context;
mod video;

use context::RetroAvCtx;
use retro_ab::core::AvInfo;
use std::sync::Arc;

pub use audio::{audio_sample_batch_callback, audio_sample_callback};
pub use video::video_refresh_callback;

#[doc = "cria uma nova instancia de RetroAvCtx. todas as instancias so podem ser criadas dentro da thread principal!"]
pub fn init(av_info: Arc<AvInfo>) -> Arc<RetroAvCtx> {
    let ctx = context::create(av_info);
    Arc::new(ctx)
}

#[doc = "eliminar o contexto atual, voce dever chamar isso sempre que nao for mais usar um contexto!"]
pub fn de_init(_av_ctx: Arc<RetroAvCtx>) {
    println!("{:?}", Arc::strong_count(&_av_ctx));

    println!("{:?}", Arc::strong_count(&_av_ctx));
}
