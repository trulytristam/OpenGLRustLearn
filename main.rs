

use glutin::event_loop::*;
use glutin::*;
use glutin::window::*;
use glutin::event::*;


fn main() {
    crazy branch
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("Learn OpenGL with Rust");

    let gl_context = ContextBuilder::new()
    .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
    .build_windowed(window, &event_loop)
    .expect("Cannot create windowed context");
    help
    let gl_context = unsafe {
    gl_context
        .make_current()
        .expect("Failed to make context current")
    };

    what the heck
   gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
        Event::LoopDestroyed => (),
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => (),
        },
        _ => (),
    }
    });

  

}
