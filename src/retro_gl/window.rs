use std::{rc::Rc, sync::Arc};

use retro_ab::core::AvInfo;
use sdl2::{
    video::{GLContext, GLProfile, Window},
    Sdl, VideoSubsystem,
};

use crate::video::RetroVideoAPi;

use super::{gl::gl, render::Render};

pub struct GlWIndow {
    _video: VideoSubsystem,
    _window: Window,
    _gl_ctx: Option<GLContext>,
    _render: Render,
    _av_info: Arc<AvInfo>,
}

impl Drop for GlWIndow {
    fn drop(&mut self) {
        //gl_ctx precisa ser deletado antes de tudo!
        /* esse é comportamento ideal aqui
        // Deletar o contexto OpenGL
        SDL_GL_DeleteContext(glcontext);

        // Destruir a janela
        SDL_DestroyWindow(window);
        */
        {
            self._gl_ctx.take();
        }

        self._video.gl_unload_library();
    }
}

impl RetroVideoAPi for GlWIndow {
    fn get_window_id(&self) -> u32 {
        self._window.id()
    }

    fn draw_new_frame(&mut self, texture: &crate::video::RawTextureData) {
        let (width, height) = self._window.size();

        self._render.draw_new_frame(
            &texture,
            &self._av_info.video.geometry,
            width as i32,
            height as i32,
        );

        self._window.gl_swap_window();
    }

    fn resize(&mut self, _new_size: (u32, u32)) {}

    fn get_proc_address(&self, procname: &str) -> *const () {
        self._video.gl_get_proc_address(procname)
    }
}

impl GlWIndow {
    pub fn new(sdl: &Sdl, av_info: &Arc<AvInfo>) -> Result<GlWIndow, String> {
        let _video = sdl.video()?;

        let gl_attr = _video.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 2);

        let geo = &av_info.video.geometry;

        let win_result = _video
            .window(
                "retro_ab_av",
                *geo.base_width.lock().unwrap(),
                *geo.base_height.lock().unwrap(),
            )
            .opengl()
            .maximized()
            .resizable()
            .position_centered()
            .build();

        match win_result {
            Ok(mut _window) => {
                let _gl_ctx = _window.gl_create_context().unwrap();
                let gl = Rc::new(gl::Gl::load_with(|name| {
                    _video.gl_get_proc_address(name) as *const _
                }));
                _video.gl_set_swap_interval(1)?;

                _window
                    .set_minimum_size(
                        *av_info.video.geometry.base_width.lock().unwrap(),
                        *av_info.video.geometry.base_height.lock().unwrap(),
                    )
                    .expect("nao e possível definir um tamanho mínimo a janela");

                let mut _render =
                    Render::new(av_info, gl.clone()).expect("erro ao tentar inciar o opengl");

                Ok(GlWIndow {
                    _video,
                    _window,
                    _gl_ctx: Some(_gl_ctx),
                    _render,
                    _av_info: av_info.clone(),
                })
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
