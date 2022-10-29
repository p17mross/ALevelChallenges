use glium::implement_vertex;

pub(crate) const CLEAR_VERTEX_SHADER: &str = "
    //VERTEX SHADER
    #version 140

    uniform mat4 positions;

    in vec2 position;

    void main() {
        gl_Position = vec4(positions[gl_VertexID]);
        //gl_Position = vec4(position, 0.0, 1.0);
    }
";

pub(crate) const CLEAR_FRAGMENT_SHADER: &str = "
    //FRAGMENT_SHADER
    #version 140

    out vec4 color;

    uniform vec4 clear_colour;

    void main() {
        color = clear_colour;
    }
";

#[derive(Copy, Clone, Debug)]
pub(crate) struct ClearVertex {
    position: [f32;2]
}

implement_vertex!(ClearVertex, position);

pub(crate) const CLEAR_VERTICES: [ClearVertex; 4] = [
    ClearVertex{position: [1.0, 1.0]},
    ClearVertex{position: [-1.0, 1.0]},
    ClearVertex{position: [1.0, -1.0]},
    ClearVertex{position: [-1.0, -1.0]},
];

pub(crate) const CLEAR_INDICES: [u32; 6] = [
    0, 1, 2,
    2, 1, 3,
];