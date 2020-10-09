#![allow(non_snake_case)]

#[macro_use]
extern crate glium;

use std::f32::consts::PI;
use glium::{
    Display,
    Surface,
    glutin
};
use glium::glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new();
    let context = ContextBuilder::new().with_srgb(false);
    let display = Display::new(window, context, &event_loop).unwrap();

    let draw_parameters: glium::DrawParameters = Default::default();
    let uniforms = uniform!{};

    let vertex_shader_src = r#"
        #version 330

        in vec3 position;

        void main() {
            gl_Position = vec4(position, 1.0);
        }
    "#;
    let fragment_shader_src = r#"
        #version 330

        out vec4 color;

        void main() {
            color = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "#;
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 3]
    }
    implement_vertex!(Vertex, position);
    let vertex_buffer = glium::VertexBuffer::new(&display, &[
        Vertex { position: [-0.5,  0.5, 0.0] },
        Vertex { position: [ 0.5,  0.5, 0.0] },
        Vertex { position: [-0.5, -0.5, 0.0] },
        Vertex { position: [ 0.5, -0.5, 0.0] },
    ]).unwrap();

    let null_time = std::time::Instant::now();
    event_loop.run(move |event, _, control_flow: &mut ControlFlow| {
        let time = std::time::Instant::now() - null_time;
        let time = time.as_secs_f32();

        let mut target = display.draw();
        let blue = (1. + f32::sin(time + PI)) / 2.;
        let red  = (1. + f32::sin(time     )) / 2.;
        target.clear_color(red, 0.0, blue, 1.0);

        target.draw(
            &vertex_buffer,
            glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
            &program,
            &uniforms,
            &draw_parameters
        ).unwrap();

        target.finish().unwrap();

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);

        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                },
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        if key == glutin::event::VirtualKeyCode::Q {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                    return;
                }
                _ => return,
            },
            _ => (),
        }
    });
}