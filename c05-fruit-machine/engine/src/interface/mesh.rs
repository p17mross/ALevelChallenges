use std::fs::File;
use std::io::BufReader;

use obj::load_obj;
use glium::implement_vertex;


#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: (f32, f32, f32),
    pub normal: (f32, f32, f32),
    pub uv: (f32, f32)
}
implement_vertex!(Vertex, position, normal, uv);

#[derive(Debug)]
pub enum ObjLoadError {
    Io(std::io::Error),
    InvalidFile
}

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub shader: Box<dyn crate::shaders::Shader>,
}

impl Mesh {
    pub fn cube(shader: Box<dyn crate::shaders::Shader>) -> Self {
        Mesh {
            //TODO: normals and uvs
            vertices: vec![
                Vertex{position: (-1.0, -1.0, -1.0), normal:(0.0, 0.0, 0.0), uv: (0.0, 0.0)},
                Vertex{position: (-1.0, -1.0, 1.0), normal:(0.0, 0.0, 0.0), uv: (0.0, 0.0)},
                Vertex{position: (-1.0, 1.0, -1.0), normal:(0.0, 0.0, 0.0), uv: (0.0, 0.0)},
                Vertex{position: (-1.0, 1.0, 1.0), normal:(0.0, 0.0, 0.0), uv: (0.0, 0.0)},
                Vertex{position: (1.0, -1.0, -1.0), normal:(0.0, 0.0, 0.0), uv: (0.0, 0.0)},
                Vertex{position: (1.0, -1.0, 1.0), normal:(0.0, 0.0, 0.0), uv: (0.0, 0.0)},
                Vertex{position: (1.0, 1.0, -1.0), normal:(0.0, 0.0, 0.0), uv: (0.0, 0.0)},
                Vertex{position: (1.0, 1.0, 1.0), normal:(0.0, 0.0, 0.0), uv: (0.0, 0.0)},
            ],
            indices: vec![
                0, 2, 3,
                0, 3, 1,
                0, 4, 6,
                0, 6, 2,
                0, 1, 5,
                0, 5, 4,
                1, 3, 7,
                1, 7, 5,
                3, 2, 6,
                3, 6, 7,
                4, 5, 7,
                4, 7, 6,
            ],
            shader:shader
         }
    }

    pub fn plane(double_sided: bool, shader: Box<dyn crate::shaders::Shader>) -> Self {
        Mesh {
            //TODO: normals
            vertices: vec![
                Vertex {position: (-1.0, 1.0, 0.0), normal: (0.0, 0.0, 0.0), uv: (0.0, 1.0)},
                Vertex {position: (1.0, 1.0, 0.0), normal: (0.0, 0.0, 0.0), uv: (1.0, 1.0)},
                Vertex {position: (-1.0, -1.0, 0.0), normal: (0.0, 0.0, 0.0), uv: (0.0, 0.0)},
                Vertex {position: (1.0, -1.0, 0.0), normal: (0.0, 0.0, 0.0), uv: (1.0, 0.0)},
            ],
            indices: match double_sided {
                true => vec![
                    0, 1, 2,
                    2, 1, 3,
                    0, 2, 1,
                    2, 3, 1,
                ],
                false => vec![
                    0, 1, 2,
                    2, 1, 3
                ]
            },
            shader:shader
        }
    }

    pub fn from_obj(path: &str, shader: Box<dyn crate::shaders::Shader>) -> Result<Self, ObjLoadError> {
        let file = match File::open(path) {
            Ok(o) => o,
            Err(e) => return Err(ObjLoadError::Io(e))
        };

        let reader = BufReader::new(file);

        // Try with position, normal and uv
        match load_obj::<obj::TexturedVertex, BufReader<File>, u32>(reader) {
            Ok(object) => return Ok(Mesh{
                vertices: object.vertices.iter().map(|vert| Vertex {
                    position:(vert.position[0], vert.position[1], vert.position[2]),
                    normal: (vert.normal[0], vert.normal[1], vert.normal[2]),
                    uv: (vert.texture[0], vert.texture[1])
                }).collect(),
                indices: object.indices.iter().map(|x|u32::from(x.to_owned())).collect(),
                shader: shader
            }),
            Err(e) => match e {
                obj::ObjError::Io(ioe) => return Err(ObjLoadError::Io(ioe)),
                _ => ()
            }
        };

        let file = match File::open(path) {
            Ok(o) => o,
            Err(e) => return Err(ObjLoadError::Io(e))
        };

        let reader = BufReader::new(file);

        // Try with position and normal only
        match load_obj::<obj::Vertex, BufReader<File>, u32>(reader) {
            Ok(object) => return Ok(Mesh{
                vertices: object.vertices.iter().map(|vert| Vertex {
                    position:(vert.position[0], vert.position[1], vert.position[2]),
                    normal: (vert.normal[0], vert.normal[1], vert.normal[2]),
                    uv: (0.0, 0.0)
                }).collect(),
                indices: object.indices.iter().map(|x|u32::from(x.to_owned())).collect(),
                shader: shader
            }),
            Err(e) => match e {
                obj::ObjError::Io(ioe) => return Err(ObjLoadError::Io(ioe)),
                _ => ()
            }
        };

        let file = match File::open(path) {
            Ok(o) => o,
            Err(e) => return Err(ObjLoadError::Io(e))
        };

        let reader = BufReader::new(file);

        // Try with position only
        match load_obj::<obj::Position, BufReader<File>, u32>(reader) {
            Ok(object) => return Ok(Mesh{
                vertices: object.vertices.iter().map(|vert| Vertex {
                    position:(vert.position[0], vert.position[1], vert.position[2]),
                    normal: (0.0, 0.0, 0.0),
                    uv: (0.0, 0.0)
                }).collect(),
                indices: object.indices.iter().map(|x|u32::from(x.to_owned())).collect(),
                shader: shader
            }),
            Err(e) => match e {
                obj::ObjError::Io(ioe) => return Err(ObjLoadError::Io(ioe)),
                _ => ()
            }
        };
        
        println!("No data");
        Err(ObjLoadError::InvalidFile)

    }
}