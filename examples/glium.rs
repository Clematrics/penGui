use glium::glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder,
};
use glium::{glutin, Display, Surface};
use std::f32::consts::PI;

use pengui::backend::glium::GliumBackend;
use pengui::core::{Backend, DrawCommand, Vertex};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new();
    let context = ContextBuilder::new().with_srgb(false);
    let display = Display::new(window, context, &event_loop).unwrap();

    let backend = GliumBackend::new(display);

    let null_time = std::time::Instant::now();
    event_loop.run(move |event, _, control_flow: &mut ControlFlow| {
        let time = std::time::Instant::now() - null_time;
        let time = time.as_secs_f32();

        let mut target = backend.new_frame();
        let blue = (1. + f32::sin(time + PI)) / 2.;
        let red = (1. + f32::sin(time)) / 2.;
        target.clear_color(red, 0.0, blue, 1.0);

        backend
            .draw_command(
                &mut target,
                &DrawCommand {
                    vertex_buffer: vec![
                        Vertex {
                            position: [-0.5, 0.5, 0.0],
                            color: [1., 0., 0., 0.],
                        },
                        Vertex {
                            position: [0.5, 0.5, 0.0],
                            color: [0., 0., 1., 0.],
                        },
                        Vertex {
                            position: [-0.5, -0.5, 0.0],
                            color: [0., 1., 0., 0.],
                        },
                        Vertex {
                            position: [0.5, -0.5, 0.0],
                            color: [0., 0., 0., 0.],
                        },
                    ],
                    index_buffer: vec![0, 1, 2, 1, 3, 2],
                    clipping: [[-1., 1.], [1., 1.]],
                    draw_mode: pengui::core::DrawMode::TriangleFan,
                    texture: 0,
                    uniforms: vec![],
                },
            )
            .unwrap();

        target.finish().unwrap();

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
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
