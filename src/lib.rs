mod audio;
mod context;
mod video;

use context::RetroAvCtx;
use retro_ab::core::AvInfo;
use std::sync::Arc;
use winit::{error::EventLoopError, event_loop::EventLoop};

pub use audio::{audio_sample_batch_callback, audio_sample_callback};
pub use video::video_refresh_callback;
pub use winit::event;
pub use winit::keyboard;

#[doc = "cria uma nova instancia de RetroAvCtx. todas as instancias so podem ser criadas dentro da thread principal!"]
pub fn init(av_info: Arc<AvInfo>) -> Result<(RetroAvCtx, EventLoop<()>), EventLoopError> {
    Ok(context::create(av_info)?)
}

#[doc = "eliminar o contexto atual, voce dever chamar isso sempre que nao for mais usar um contexto!"]
pub fn de_init(_av_ctx: RetroAvCtx) {}
