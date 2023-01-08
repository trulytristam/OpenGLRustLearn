use std::fs;
use std::net::SocketAddr;
use std::error::Error;
mod render;
use render::*;
use std::borrow::Cow;
extern crate glium;

fn main() {
    use glium::{glutin, Surface};
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb,cb,&event_loop).unwrap();

    #[derive(Clone, Copy)]
    struct vert{
        pos: [f32; 3],
    }  
    glium::implement_vertex!(vert,pos);
    let vs  = [vert{pos: [-1.0,-1.0,1.0]},
                          vert{pos: [-1.0,1.0,1.0]},
                          vert{pos: [1.0,1.0,1.0]},
                          vert{pos: [1.0,-1.0,1.0]}];
    let vi: &[u32;6] = &[0,3,2,2,1,0];
    //define uniform

    let vertex_buffer = glium::vertex::VertexBuffer::new(&display,&vs).unwrap();
    vertex_buffer.vertex_attrib();
    let index_buffer = glium::index::IndexBuffer::new(&display,glium::index::PrimitiveType::TrianglesList, vi).unwrap();
    let vertex_source = fs::read_to_string("vertex_shader.glsl").unwrap();
    let fragment_source = fs::read_to_string("fragment_shader.glsl").unwrap();
    let geometry_source = &fs::read_to_string("gemotry.glsl").unwrap();
    let program = glium::Program::from_source(&display,&vertex_source,&fragment_source,None).unwrap();


    let query = glium::draw_parameters::SamplesPassedQuery::new(&display).unwrap();
    let params = glium::DrawParameters {
    depth: glium::Depth {
        test: glium::DepthTest::IfLess,
        write: true,
        .. Default::default()
    },
    .. Default::default()
    };
    let uniforms = glium::uniform! {
    };
    
    event_loop.run(move |ev, _, control_flow| {
    
            let mut target = display.draw();
            //target.clear_color(1.0, 0.0, 1.0, 0.0);
            let tar = target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &params);
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
