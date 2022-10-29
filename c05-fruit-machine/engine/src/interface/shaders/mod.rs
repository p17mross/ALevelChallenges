pub(crate) mod clear;

#[allow(non_snake_case)]
mod unshaded_3D;
#[allow(non_snake_case)]
mod unshaded_2D;
#[allow(non_snake_case)]
mod texture_only_3D;
#[allow(non_snake_case)]
mod texture_only_2D;

use std::{rc::Rc};

pub use unshaded_3D::Unshaded3D;
pub use unshaded_2D::Unshaded2D;
pub use texture_only_3D::TextureOnly3D;
pub use texture_only_2D::TextureOnly2D;

pub(crate) use clear::*;

use crate::Window;

use self::shader_priv::{TexturePriv, TextureLoadError};

pub(crate) mod shader_priv {
    use std::{fmt::Debug};
    use glium::{Program, uniforms::{Uniforms, UniformValue}, Display, ProgramCreationError};

    use crate::Transform;

    use super::Texture;

    #[derive(Clone)]
    pub enum UniformType<'a> {
        Immediate(UniformValue<'a>),
        Texture(Texture),
    }

    #[derive(Clone)]
    pub struct ShaderUniforms<'a> (pub Vec<(String, UniformType<'a>)>);

    impl<'a> Uniforms for ShaderUniforms<'a> {
        fn visit_values<'b, F: FnMut(&str, UniformValue<'b>)>(&'b self, mut output: F) {
            for pair in &self.0 {
                let st;
                let uv;
                match pair {
                    (s, UniformType::Immediate(u)) => (st, uv) = (s, *u),
                    (s, UniformType::Texture(t)) => {
                        (st, uv) = (s, UniformValue::SrgbTexture2d(&t.0.0, Default::default()))
                    }

                }

                output(st, uv);
            }
        }
    }

    #[derive(Debug)]
    pub enum TextureLoadError {
        Io(std::io::Error),
        ImageError,
        Os(std::io::Error)
    }

    #[derive(Debug)]
    pub enum AssetCreationError {
        Program(ProgramCreationError),
        Texture(TextureLoadError)
    }

    #[derive(Debug)]
    pub struct TexturePriv (pub glium::texture::SrgbTexture2d);

    pub trait ShaderPriv: Debug {
        fn get_vertex_shader(&self) -> String;
        fn get_fragment_shader(&self) -> String;
        fn get_program(& self) -> &Option<Program>;
        fn create_assets(&mut self, display: &Display) -> Result<(), AssetCreationError>;
        fn get_uniforms(&self, camera_mat: &Transform, mesh_mat: &Transform, fov: f32, aspect_ratio: f32, zfar: f32, znear: f32, obj_mat: &crate::Transform, out: &mut ShaderUniforms);
    }
}
pub trait Shader: shader_priv::ShaderPriv {}

#[derive(Debug, Clone)]
pub struct Texture(Rc<TexturePriv>);

impl Texture {
    pub fn new(path: String, window: &Window) -> Result<Self, TextureLoadError> {
        let texture_image = match image::open(path) {
            Ok(i) => i,
            Err(e) => match e {
                image::ImageError::IoError(e) => return Err(TextureLoadError::Io(e)),
                _ => return Err(TextureLoadError::ImageError)
            }
        }.to_rgba8();
        let image_dimensions = texture_image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&texture_image.into_raw(), image_dimensions);

        let texture = match glium::texture::SrgbTexture2d::new(&window.display, image) {
            Ok(t) => t,
            Err(_) => return Err(TextureLoadError::ImageError)
        };

        Ok(Texture(Rc::new(TexturePriv(texture))))
    }
}