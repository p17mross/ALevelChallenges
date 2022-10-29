use glium::{Program, uniforms::UniformValue};

use crate::Transform;

use super::{shader_priv::{ShaderUniforms, ShaderPriv, UniformType, AssetCreationError}, Shader, Texture};

pub(crate) const VERTEX_SHADER_TEXTURE_ONLY_3D_SRC: &str = r#"
    #version 140

    in vec3 position;
    in vec3 normal;
    in vec2 uv;

    out vec2 v_uv;

    uniform mat4 camera_matrix;
    uniform mat4 perspective_matrix;
    uniform mat4 object_matrix;
    uniform mat4 mesh_matrix;

    void main() {
        v_uv = uv;
        gl_Position = perspective_matrix * inverse(camera_matrix) * object_matrix * mesh_matrix * vec4(position, 1.0);
    }
"#;

pub(crate) const FRAGMENT_SHADER_TEXTURE_ONLY_3D_SRC: &str = r#"
    #version 140

    in vec2 v_uv;

    out vec4 colour_out;

    uniform sampler2D tex;

    void main() {
        colour_out = texture(tex, v_uv);
    }
"#;

struct TextureOnly3DPriv {
    vertex_text: String,
    fragment_text: String,
    program: Option<Program>,
    texture: Texture
}

impl TextureOnly3DPriv {
    fn create_program(&mut self, display: &glium::Display) -> Result<&Program, glium::ProgramCreationError> {
        self.program = Some(glium::Program::from_source(display, &self.vertex_text, &self.fragment_text, None)?);
        Ok(self.program.as_ref().unwrap())
    }
}

impl<'a> std::fmt::Debug for TextureOnly3DPriv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "program generated: {}", self.program.is_some()).unwrap();
        Ok(())
    }
}

impl<'a> ShaderPriv for TextureOnly3DPriv {
    fn get_uniforms(&self, camera_mat: &Transform, mesh_mat: &Transform, fov: f32, aspect_ratio: f32, zfar: f32, znear: f32, obj_mat: &crate::Transform, out: &mut ShaderUniforms) {

        let persp_mat = {
            let f = 1.0 / ((3.141592 / fov) / 2.0).tan();
        
            [
                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ]
        };

        *out = ShaderUniforms (
            vec![
                ("camera_matrix".to_string(), UniformType::Immediate(UniformValue::Mat4(camera_mat.to_array()))),
                ("perspective_matrix".to_string(), UniformType::Immediate(UniformValue::Mat4(persp_mat))),
                ("object_matrix".to_string(), UniformType::Immediate(UniformValue::Mat4(obj_mat.to_array()))),
                ("mesh_matrix".to_string(), UniformType::Immediate(UniformValue::Mat4(mesh_mat.to_array()))),
                ("tex".to_string(), UniformType::Texture(self.texture.clone())),
            ]
        )
        
    }
    fn get_vertex_shader(&self) -> String {
        self.vertex_text.clone()
    }
    fn get_fragment_shader(&self) -> String {
        self.fragment_text.clone()
    }
    fn get_program(&self) -> &Option<Program> {
        &self.program
    }
    fn create_assets(&mut self, display: &glium::Display) -> Result<(), AssetCreationError> {
        if self.program.is_none() {
            return match self.create_program(display) {
                Ok(_) => Ok(()),
                Err(e) => Err(AssetCreationError::Program(e))
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct TextureOnly3D (TextureOnly3DPriv);

impl TextureOnly3D {
    pub fn new(texture: Texture) -> Self {
        TextureOnly3D(TextureOnly3DPriv {
            vertex_text: VERTEX_SHADER_TEXTURE_ONLY_3D_SRC.to_string(),
            fragment_text: FRAGMENT_SHADER_TEXTURE_ONLY_3D_SRC.to_string(),
            program: None,
            
            texture: texture,
        })
    }
}


impl ShaderPriv for TextureOnly3D {
    fn get_uniforms(&self, camera_mat: &Transform, mesh_mat: &Transform, fov: f32, aspect_ratio: f32, zfar: f32, znear: f32, obj_mat: &crate::Transform, out: &mut ShaderUniforms) {
        self.0.get_uniforms(camera_mat, mesh_mat, fov, aspect_ratio, zfar, znear, obj_mat, out)
    }
    fn get_fragment_shader(&self) -> String {
        self.0.get_fragment_shader()
    }
    fn get_vertex_shader(&self) -> String {
        self.0.get_vertex_shader()
    }
    fn get_program(&self) -> &Option<Program> {
        self.0.get_program()
    }
    fn create_assets(&mut self, display: &glium::Display) -> Result<(), AssetCreationError> {
        self.0.create_assets(display)
    }
}
impl Shader for TextureOnly3D {}