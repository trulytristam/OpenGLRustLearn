

extern crate nalgebra;
extern crate glium;
mod object_manager;
use std::fs;
use nalgebra::Vector3;
fn main(){

    use glium::{glutin, Surface};
    let OM = object_manager::ObjectManager::new();

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
        // Create a window
    let display = glium::Display::new(wb,cb,&event_loop).unwrap();
    let ubP = glium::uniforms::UniformBuffer::new(&display,OM.getObjectPositions()).unwrap();
    let ubO = glium::uniforms::UniformBuffer::new(&display,OM.getObjectOrientations()).unwrap();

    // Create a vertex buffer
    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
        }
        glium::implement_vertex!(Vertex,position);
        glium::VertexBuffer::new(&display, 
            &[
                Vertex { position: [-1.0,  1.0] },
                Vertex { position: [-1.0, -1.0] },
                Vertex { position: [ 1.0, -1.0] },
                Vertex { position: [ 1.0,  1.0] },

            ]
        ).unwrap()
    };
    // Create an index buffer
    let index_buffer = glium::IndexBuffer::new(&display,
        glium::index::PrimitiveType::TrianglesList,
        &[0u16, 1, 2, 2, 3, 0]
    ).unwrap();
    let fragment_source = std::fs::read_to_string("fragment_shader.glsl").unwrap();
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
        &fragment_source,
    None).unwrap();
    let ws = display.get_framebuffer_dimensions(); 
    
    
    
    // Main loop
    
        // Draw the triangle
    event_loop.run(move |ev, _, control_flow| {    
        let uniforms = glium::uniform! {
            windowSizeX: ws.0,
            windowSizeY: ws.1,
            positions: &ubP,
            orientations: &ubO,};
        let mut target = display.draw();
        target.draw(&vertex_buffer, &index_buffer, &program,
                    &uniforms, 
                    &Default::default()).unwrap();
        target.finish().unwrap();
        let next_frame_time = std::time::Instant::now() +
                std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }
    });
    
}
