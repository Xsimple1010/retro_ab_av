use std::{rc::Rc, sync::Arc};

use retro_ab::core::AvInfo;
use sdl2::{
    video::{GLContext, GLProfile, Window},
    Sdl, VideoSubsystem,
};

use crate::video::RetroVideoAPi;

use super::{gl::gl, render::Render};

pub struct GlWIndow {
    video: VideoSubsystem,
    window: Window,
    gl_ctx: Option<GLContext>,
    render: Render,
    av_info: Arc<AvInfo>,
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
            self.gl_ctx.take();
        }

        self.video.gl_unload_library();
    }
}

impl RetroVideoAPi for GlWIndow {
    fn get_window_id(&self) -> u32 {
        self.window.id()
    }

    fn draw_new_frame(&mut self, texture: &crate::video::RawTextureData) {
        let (width, height) = self.window.size();

        self.render.draw_new_frame(
            &texture,
            &self.av_info.video.geometry,
            width as i32,
            height as i32,
        );

        self.window.gl_swap_window();
    }

    fn resize(&mut self, (width, height): (u32, u32)) {
        self.window.set_size(width, height).unwrap();
    }

    fn get_proc_address(&self, proc_name: &str) -> *const () {
        self.video.gl_get_proc_address(proc_name)
    }
}

impl GlWIndow {
    pub fn new(sdl: &Sdl, av_info: &Arc<AvInfo>) -> Result<GlWIndow, String> {
        let video = sdl.video()?;

        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 2);

        let geo = &av_info.video.geometry;

        let win_result = video
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
            Ok(mut window) => {
                let gl_ctx = window.gl_create_context().unwrap();
                let gl = Rc::new(gl::Gl::load_with(|name| {
                    video.gl_get_proc_address(name) as *const _
                }));
                video.gl_set_swap_interval(1)?;

                window
                    .set_minimum_size(
                        *av_info.video.geometry.base_width.lock().unwrap(),
                        *av_info.video.geometry.base_height.lock().unwrap(),
                    )
                    .expect("nao e possível definir um tamanho mínimo a janela");

                let render =
                    Render::new(av_info, gl.clone()).expect("erro ao tentar inciar o opengl");

                Ok(GlWIndow {
                    video,
                    window,
                    gl_ctx: Some(gl_ctx),
                    render,
                    av_info: av_info.clone(),
                })
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
