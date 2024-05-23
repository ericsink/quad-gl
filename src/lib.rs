#![allow(warnings)]
use slotmap::SlotMap;
use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::pin::Pin;

mod quad_gl;

pub mod camera;
pub mod color;
pub mod material;
pub mod math;
pub mod models;
pub mod shapes;
pub mod text;
pub mod texture;
pub mod ui;

// ERIC
// I found some rounded rect code in a macroquad Pr
pub mod rounded_rect;

pub mod telemetry;

mod cubemap;
mod error;
pub mod shadowmap;

pub use error::Error;

pub mod scene;
pub mod sprite_batcher;

pub(crate) mod image;

use crate::{
    color::{colors::*, Color},
    quad_gl::QuadGl,
    texture::TextureHandle,
};

use glam::{vec2, Mat4, Vec2};
use std::sync::{Arc, Mutex, Weak};

// pub(crate) fn pixel_perfect_projection_matrix(&self) -> glam::Mat4 {
//     let (width, height) = miniquad::window::screen_size();

//     let dpi = miniquad::window::dpi_scale();

//     glam::Mat4::orthographic_rh_gl(0., width / dpi, height / dpi, 0., -1., 1.)
// }

// pub(crate) fn projection_matrix(&self) -> glam::Mat4 {
//     if let Some(matrix) = self.camera_matrix {
//         matrix
//     } else {
//         self.pixel_perfect_projection_matrix()
//     }
// }

#[derive(Clone)]
pub struct Context3 {
    pub quad_ctx: Arc<Mutex<Box<miniquad::Context>>>,
    textures: Arc<Mutex<crate::texture::TexturesContext>>,
    fonts_storage: Arc<Mutex<text::FontsStorage>>,
}

impl Context3 {
    pub fn new(quad_ctx: Arc<Mutex<Box<miniquad::Context>>>) -> Context3 {
        let fonts_storage = text::FontsStorage::new(quad_ctx.lock().unwrap().as_mut());
        let textures = crate::texture::TexturesContext::new();
        Context3 {
            quad_ctx,
            fonts_storage: Arc::new(Mutex::new(fonts_storage)),
            textures: Arc::new(Mutex::new(textures)),
        }
    }

    pub fn new_scene(&self) -> scene::Scene {
        scene::Scene::new(self.quad_ctx.clone(), self.fonts_storage.clone())
    }

    pub fn new_canvas(&self) -> sprite_batcher::SpriteBatcher {
        sprite_batcher::SpriteBatcher::new(self.quad_ctx.clone(), self.fonts_storage.clone())
    }

    // ERIC
    // I found this function in text.rs, commented out.
    // This seems like a decent place for it to go?
    pub fn load_ttf_font_from_bytes(
        &self,
        bytes: &[u8]
        ) -> Result<crate::text::Font, Error>
    {
        let atlas = Arc::new(Mutex::new(crate::text::atlas::Atlas::new(
            self.quad_ctx.lock().unwrap().as_mut(),
            miniquad::FilterMode::Linear,
        )));

        let mut font = crate::text::Font::load_from_bytes(atlas.clone(), bytes)?;

        font.populate_font_cache(&crate::text::Font::ascii_character_list(), 15);

        Ok(font)
    }

}

// pub fn start<F: Fn(Context3) -> Fut + 'static, Fut: Future<Output = ()> + 'static>(
//     mut config: conf::Conf,
//     future: F,
// ) {
//     miniquad::start(conf::Conf { ..config }, move || {
//         let ctx = Context3::new();
//         Box::new(Stage {
//             main_future: Some(Box::pin(future(ctx.clone()))),
//             ctx: Arc::new(ctx),
//         })
//     });
// }
