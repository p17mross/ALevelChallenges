use glium::{uniforms::UniformValue, Program};

use crate::Transform;

use super::{shader_priv::{ShaderUniforms, ShaderPriv, UniformType, AssetCreationError}, Shader};

pub(crate) const VERTEX_SHADER_UNSHADED_2D_SRC: &str = r#"
    #version 140

    in vec3 position;
    in vec3 normal;

    uniform mat4 camera_matrix;
    uniform mat4 perspective_matrix;
    uniform mat4 object_matrix;
    uniform mat4 mesh_matrix;

    void main() {
        gl_Position = perspective_matrix * inverse(camera_matrix) * object_matrix * mesh_matrix * vec4(position, 1.0);
    }
"#;

pub(crate) const FRAGMENT_SHADER_UNSHADED_2D_SRC: &str = r#"
    #version 140

    out vec4 colour_out;
    uniform vec4 colour_in;

    void main() {
        colour_out = vec4(colour_in);
    }
"#;

#[derive(Debug)]
struct Unshaded2DPriv {
    vertex_text: String,
    fragment_text: String,
    program: Option<Program>,
    colour: [f32; 4],
}

impl Unshaded2DPriv {
    fn create_program(&mut self, display: &glium::Display) -> Result<&Program, glium::ProgramCreationError> {
        self.program = Some(glium::Program::from_source(display, &self.vertex_text, &self.fragment_text, None)?);
        Ok(self.program.as_ref().unwrap())
    }
}

impl ShaderPriv for Unshaded2DPriv {
    fn get_uniforms(&self, camera_mat: &Transform, mesh_mat: &Transform, _fov: f32, aspect_ratio: f32, zfar: f32, _znear: f32, obj_mat: &crate::Transform, out: &mut ShaderUniforms) {
        let persp_mat = {
        
            [
                [ zfar * (aspect_ratio).min(1.0) ,                 0.0                   ,       0.0       ,   0.0   ],
                [                0.0             , zfar *  (1.0 / aspect_ratio).min(1.0) ,       0.0       ,   0.0   ],
                [                0.0             ,                 0.0                   ,       1.0       ,   0.0   ],
                [                0.0             ,                 0.0                   ,       0.0       ,   zfar  ],
            ]
        };

        *out = ShaderUniforms (
            vec![
                ("camera_matrix".to_string(), UniformType::Immediate(UniformValue::Mat4(camera_mat.to_array()))),
                ("perspective_matrix".to_string(), UniformType::Immediate(UniformValue::Mat4(persp_mat))),
                ("object_matrix".to_string(), UniformType::Immediate(UniformValue::Mat4(obj_mat.to_array()))),
                ("mesh_matrix".to_string(), UniformType::Immediate(UniformValue::Mat4(mesh_mat.to_array()))),
                ("colour_in".to_string(), UniformType::Immediate(UniformValue::Vec4(self.colour))),
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
pub struct Unshaded2D (Unshaded2DPriv);

impl Unshaded2D {
    pub fn new(colour: [f32; 4]) -> Self {
        Unshaded2D (
            Unshaded2DPriv { 
                vertex_text: VERTEX_SHADER_UNSHADED_2D_SRC.to_string(),
                fragment_text: FRAGMENT_SHADER_UNSHADED_2D_SRC.to_string(),
                program: None,
                
                colour: colour
            }  
        )
        
    }
}


impl ShaderPriv for Unshaded2D {
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
impl Shader for Unshaded2D {}