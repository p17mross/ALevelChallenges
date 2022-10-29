use std::fmt::Debug;

use glium::Frame;
use glium::IndexBuffer;
use glium::Program;
use glium::Rect;
use glium::Surface;
use glium::VertexBuffer;
use glium::uniform;

use crate::Scene;
use crate::Transform;
use crate::Window;
use crate::shaders;
use crate::shaders::shader_priv::ShaderUniforms;

#[derive(Debug)]
pub enum Renderable {
    Camera(Camera),
    SplitView(SplitView),
}

impl Renderable {
    pub(crate) fn render(&mut self, frame: &mut Frame, scene: &mut Scene, window: &Window, x_start: f32, x_end: f32, y_start: f32, y_end: f32) {
        match self {
            Renderable::Camera(c) => c.render(frame, scene, window, x_start, x_end, y_start, y_end),
            Renderable::SplitView(s) => s.render(frame, scene, window, x_start, x_end, y_start, y_end)
        }
    }
}

#[derive(Debug)]
pub struct Camera {
    clear_colour: Option<[f32; 4]>,
    pub transform: Transform,
    pub fov: f32,
    pub znear: f32,
    pub zfar: f32,

    #[doc(hidden)]
    clear_vertices: Option<VertexBuffer<shaders::ClearVertex>>,
    clear_program: Option<Program>,
    clear_indices: Option<IndexBuffer<u32>>,
}


impl Camera {

    pub fn set_clear_colour (&mut self, colour: Option<[f32; 4]>) {
        self.clear_colour = colour;
        self.clear_vertices = None;
        self.clear_program = None;
        self.clear_indices = None;
    }

    pub fn new(transform: Transform, fov: f32) -> Self {
        Camera { 
            clear_colour: None,
            transform: transform,
            fov: fov,
            znear: 0.1,
            zfar: 1024.0,


            clear_vertices: None,
            clear_program: None,
            clear_indices: None,
        }
    }

    fn render(&mut self, frame: &mut Frame, scene: &mut Scene, window: &Window, x_start: f32, x_end: f32, y_start: f32, y_end: f32) {
        
        frame.clear_depth(1.0);

        if let Some(clear_colour) = self.clear_colour {
            //Create program to draw single colour rectangle to screen
            if self.clear_program.is_none() {
                self.clear_program = Some(glium::Program::from_source(&window.display, shaders::CLEAR_VERTEX_SHADER, shaders::CLEAR_FRAGMENT_SHADER, None).unwrap());
                self.clear_vertices = Some(glium::VertexBuffer::new(&window.display, &shaders::CLEAR_VERTICES).unwrap());
                self.clear_indices = Some(glium::IndexBuffer::new(&window.display, glium::index::PrimitiveType::TrianglesList, &shaders::CLEAR_INDICES).unwrap());
            }

            let uniforms = uniform! {
                positions: [
                    [x_end, y_end, 0.0, 1.0],
                    [x_start, y_end, 0.0, 1.0],
                    [x_end, y_start, 0.0, 1.0],
                    [x_start, y_start, 0.0, 1.0]
                ],
                clear_colour:clear_colour,             
            };

            frame.draw(self.clear_vertices.as_ref().unwrap(), self.clear_indices.as_ref().unwrap(), self.clear_program.as_ref().unwrap(), &uniforms, &Default::default()).unwrap();
        }

        let (width, height) = frame.get_dimensions();

        let x_start_mapped = ((x_start + 1.0) / 2.0 * width as f32) as u32;
        let x_end_mapped = ((x_end + 1.0) / 2.0 * width as f32) as u32;
        let y_start_mapped = ((y_start + 1.0) / 2.0 * height as f32) as u32;
        let y_end_mapped = ((y_end + 1.0) / 2.0 * height as f32) as u32;

        let aspect_ratio = ((y_end_mapped - y_start_mapped) as f32) / ((x_end_mapped - x_start_mapped) as f32);

        for object in &mut scene.objects {
            for mesh in &mut object.meshes {

                let positions = glium::VertexBuffer::new(&window.display, &mesh.1.vertices).unwrap();
                let indices = glium::IndexBuffer::new(&window.display, glium::index::PrimitiveType::TrianglesList,
                                                    &mesh.1.indices).unwrap();


                let params = glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        .. Default::default()
                    },
                    backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                    viewport: Some(Rect{left: x_start_mapped, bottom: y_start_mapped, width: x_end_mapped - x_start_mapped, height: y_end_mapped - y_start_mapped}),
                    scissor: Some(Rect{left: x_start_mapped, bottom: y_start_mapped, width: x_end_mapped - x_start_mapped, height: y_end_mapped - y_start_mapped}),
                    .. Default::default()
                };

                let (program, uniforms) = {
                    let shader = &mut mesh.1.shader;
                    let mut uniforms= ShaderUniforms(vec![]);

                    shader.get_uniforms(&self.transform, &mesh.0, self.fov, aspect_ratio, self.zfar, self.znear, &object.transform, &mut uniforms);

                    shader.create_assets(&window.display).unwrap();

                    let program = shader.get_program();
                    (program, uniforms)
                };

                frame.draw(&positions, &indices, &program.as_ref().unwrap(), &ShaderUniforms::from(uniforms), &params).unwrap();

            }
        }
    }
}

/// A struct allowing multiple cameras in a scene to draw to one window
#[derive(Debug)]
pub struct SplitView {
    pub views: Vec<(Renderable, f32, f32, f32, f32)>
}

impl SplitView {
    //TODO: test this logic
    fn render(&mut self, frame: &mut Frame, scene: &mut Scene, window: &Window, x_start: f32, x_end: f32, y_start: f32, y_end: f32) {
        let mx = (x_end - x_start) / 2.0;
        let cx = (x_end + x_start) / 2.0;
        let my = (y_end - y_start) / 2.0;
        let cy = (y_end + y_start) / 2.0;
        for view in self.views.iter_mut() {
            let new_x_start = mx * view.1 + cx;
            let new_x_end = mx * view.2 + cx;
            let new_y_start = my * view.3 + cy;
            let new_y_end = my * view.4 + cy;

            view.0.render(frame, scene, window, new_x_start, new_x_end, new_y_start, new_y_end);
        }
    }
}