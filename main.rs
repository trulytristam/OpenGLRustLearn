extern crate nalgebra;
extern crate glium;
use glium::Surface;
mod object_manager;
use nalgebra::Vector3;
use nalgebra::Matrix3;
fn main(){
    let a = std::time::Instant::now();
    //init cam
    let cam_pos = [0f32,0.,0.]; 
    let cam_ori = Matrix3::<f64>::default();
    
    //use glium::{glutin, Surface};
    let mut om = object_manager::ObjectManager::new();

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
        // Create a window
    let display = glium::Display::new(wb,cb,&event_loop).unwrap();

    let vertex_buffer = {
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
    let vertex_source = std::fs::read_to_string("vertex_shader.glsl").unwrap();
    // Create a program
    let program = glium::Program::from_source(&display,&vertex_source,&fragment_source,None).unwrap();
    
    
    let test = om.get_object_orientations()[0]; 
    // println!("{:?}", test);
    // Main loop
    
        // Draw the triangle
    event_loop.run(move |ev, _, control_flow| {    
        let ub_pos = glium::uniforms::UniformBuffer::new(&display,om.get_object_position()).unwrap();
        // println!("from main: {:?}", OM.getObjectOrientations()[0] );
        let ub_o = glium::uniforms::UniformBuffer::new(&display,om.get_object_orientations()).unwrap();
        let debug_line_colors = glium::uniforms::UniformBuffer::new(&display,om.debug.get_line_colors() ).unwrap();
        let ub_o_dim = glium::uniforms::UniformBuffer::new(&display,om.get_object_dims()).unwrap();
        let current_time = std::time::Instant::now().duration_since(a).as_secs_f64();
        let ws = display.get_framebuffer_dimensions(); 
        let uniforms = glium::uniform! {
            windowSizeX: ws.0,
            windowSizeY: ws.1,
            cPos: cam_pos,
            iTime: current_time, 
            object_count: om.get_len(),
            lineColors: &debug_line_colors,
            positions: &ub_pos,
            dims: &ub_o_dim,
            orientations: &ub_o};
        let mut target = display.draw();
        target.draw(&vertex_buffer, &index_buffer, &program,&uniforms,&Default::default()).unwrap();
        let (debug_p,debug_v) = get_debug_program(&display,  &mut om);

        let param = glium::DrawParameters{
            line_width: Some(3.0),
            ..Default::default()
        };
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);
        target.draw(&debug_v, &indices, &debug_p ,&uniforms,&param).unwrap();
        // println!("matMain: {:?}", OM.getObjectOrientations()[0]);
        target.finish().unwrap();
        let next_frame_time = std::time::Instant::now() +
                std::time::Duration::from_nanos(16_666_667);
        // println!("current time: {}", currentTime);
        om.update(0.016666,current_time,(ws.0 as f64, ws.1 as f64));

    
        // println!("line #: {:?}", OM.debug.lines[0]);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        // println!("len: {:?}", OM.getLen());
       
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => {process_input(&mut om, &event);
                match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    },


                    _ => return,
                }},
            _ => (),

        }
    });
    
}

use glium::glutin;
fn process_input(om: &mut object_manager::ObjectManager, event: &glutin::event::WindowEvent<'_>){
    let input = match *event {
        glutin::event::WindowEvent::KeyboardInput {input, ..} => input,
        _ => return,
    };
    let pressed = input.state == glutin::event::ElementState::Pressed;
    let key = match input.virtual_keycode {
        Some(key) => key,
        None => return,
    };
    let speed = 5.5f64;
    let aspeed = 3.5f64;
    let forward =om.cam.1* Vector3::<f64>::new(0.,0.,1.);
    let right =om.cam.1* Vector3::<f64>::new(1.,0.,0.);
    match key {
        glutin::event::VirtualKeyCode::A => om.cam.0 -= right*speed*0.016666, 
        glutin::event::VirtualKeyCode::D => om.cam.0 += right*speed*0.016666, 
        glutin::event::VirtualKeyCode::W => om.cam.0 += forward*speed*0.016666, 
        glutin::event::VirtualKeyCode::S => om.cam.0 -= forward*speed*0.016666,  
        glutin::event::VirtualKeyCode::Left => om.cam.1 *= nalgebra::UnitQuaternion::<f64>::new(Vector3::<f64>::new(0.,1.,0.)*-aspeed*0.016666), 
        glutin::event::VirtualKeyCode::Right=> om.cam.1 *= nalgebra::UnitQuaternion::<f64>::new(Vector3::<f64>::new(0.,1.,0.)*aspeed*0.016666), 
        _ => (),
    };

    // println!("{:?}", OM.cam.0);
}


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
// Create a vertex buffer
glium::implement_vertex!(Vertex,position);

#[derive(Copy, Clone)]
struct VertexLine {
    position: [f32; 2],
    color: [f32;3],
}
// Create a vertex buffer
glium::implement_vertex!(VertexLine,position, color);




fn get_debug_program(display:& glium::Display, om: &mut object_manager::ObjectManager)->(glium::Program, glium::VertexBuffer<VertexLine>){
    let lines = om.debug.getlines(om.screen_dim,om.cam);
    let linecolors = om.debug.get_line_colors();
    // println!("{:?}", linecolors);
    let mut debuglines: Vec<VertexLine> = vec![];
    let mut debug_index_data:Vec<u16> = vec![];
    let mut dim = (display.get_framebuffer_dimensions().0 as f32,display.get_framebuffer_dimensions().0 as f32) ; 
    
    for i in 0..256{
        // debuglines.push(Vertex {position: [lines[i].0,(dim.1/dim.0)*lines[i].1]});  
        let j = i*4;
        debuglines.push(VertexLine {position: [lines[j] as f32, lines[j+1] as f32], color: [linecolors[i*3] as f32,linecolors[i*3+1] as f32,linecolors[i*3+2] as f32]});  
        debuglines.push(VertexLine {position: [lines[j+2] as f32,  lines[j+3] as f32], color: [linecolors[i*3] as f32,linecolors[i*3+1] as f32,linecolors[i*3+2] as f32]});  
        debug_index_data.push(i as u16);
    }
    let debug_vertex_buffer = glium::VertexBuffer::new(display, &debuglines).unwrap(); 
    // let debug_index_buffer = glium::IndexBuffer::new(display,glium::index::PrimitiveType::LinesList,&debug_index_data).unwrap();
    let debug_frag = std::fs::read_to_string("debugFrag.glsl").unwrap();
    let debug_vert = std::fs::read_to_string("debugVert.glsl").unwrap();
    let debug_program = glium::Program::from_source(display,&debug_vert,&debug_frag,None).unwrap();


    (debug_program, debug_vertex_buffer)

}
