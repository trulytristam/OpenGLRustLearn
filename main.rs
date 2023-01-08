// Load the Glium library
extern crate glium;

fn main(){

    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
        // Create a window
    let display = glium::Display::new(wb,cb,&event_loop).unwrap();

    // Create a vertex buffer
    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
        }
        glium::implement_vertex!(Vertex,position);
        glium::VertexBuffer::new(&display, 
            &[
                Vertex { position: [-0.5, -0.5] },
                Vertex { position: [ 0.0,  0.5] },
                Vertex { position: [ 0.5, -0.5] },
            ]
        ).unwrap()
    };

    // Create an index buffer
    let index_buffer = glium::IndexBuffer::new(&display,
        glium::index::PrimitiveType::TrianglesList,
        &[0u16, 1, 2]
    ).unwrap();

    // Create a program
    let program = glium::Program::from_source(&display,
        // vertex shader
        "
            #version 140

            in vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        ",

        // fragment shader
        "
            #version 140

            out vec4 color;

            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        ",
    None).unwrap();

    // Main loop
    loop {
        // Draw the triangle
        let mut target = display.draw();
        target.draw(&vertex_buffer, &index_buffer, &program,
                    &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

 
    }
}
