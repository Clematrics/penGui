use glium::glutin::{
    event::{Event, DeviceEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder,
};
use glium::{glutin, Display, Surface};
use std::f32::consts::PI;

use pengui::backend::glium::GliumBackend;
use pengui::core::{Backend, DrawCommand, Uniform, Vertex};

use nalgebra;

fn to_array(mat: &nalgebra::Matrix4<f32>) -> [[f32; 4]; 4] {
    [
        [mat[(0, 0)], mat[(0, 1)], mat[(0, 2)], mat[(0, 3)]],
        [mat[(1, 0)], mat[(1, 1)], mat[(1, 2)], mat[(1, 3)]],
        [mat[(2, 0)], mat[(2, 1)], mat[(2, 2)], mat[(2, 3)]],
        [mat[(3, 0)], mat[(3, 1)], mat[(3, 2)], mat[(3, 3)]],
    ]
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new();
    let context = ContextBuilder::new().with_srgb(false);
    let display = Display::new(window, context, &event_loop).unwrap();

    let backend = GliumBackend::new(display);

    let null_time = std::time::Instant::now();

	let mut view_angle_y = PI;
	let mut view_angle_x = 0.;

	let model = nalgebra::Matrix4::identity();

    event_loop.run(move |event, _, control_flow: &mut ControlFlow| {
        let time = std::time::Instant::now() - null_time;
        let time = time.as_secs_f32();

        let mut target = backend.new_frame();
        let blue = (1. + f32::sin(time + PI)) / 2.;
        let red = (1. + f32::sin(time)) / 2.;
        target.clear_color(red, 0.0, blue, 1.0);

        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = width as f32 / height as f32;

            let fov = PI / 4.0;
            let far = 8192.;
            let near = 1.;

            nalgebra::Matrix4::new_perspective(aspect_ratio, fov, near, far)
		};

		let view = {
			let x: nalgebra::Vector3<f32> = nalgebra::Vector3::x();
			let z: nalgebra::Vector3<f32> = nalgebra::Vector3::z();

			let rot_y = nalgebra::UnitQuaternion::from_axis_angle(&nalgebra::Vector3::y_axis(), view_angle_y);
			let x = rot_y.transform_vector(&x);
			let z = rot_y.transform_vector(&z);

			let rot_x = nalgebra::UnitQuaternion::from_axis_angle(&nalgebra::Unit::new_normalize(x), view_angle_x);
			let z = rot_x.transform_vector(&z);

			let eye = nalgebra::Point3::new(0., 0., 1.);
			let target = eye + z;
			nalgebra::Isometry3::look_at_rh(&eye, &target, &nalgebra::Vector3::y_axis()).to_homogeneous()
		};

        backend
            .draw_command(
                &mut target,
                &DrawCommand {
                    vertex_buffer: vec![
                        Vertex {
                            position: [-0.5, 0.5, -1.0],
                            color: [1., 0., 0., 0.],
                        },
                        Vertex {
                            position: [0.5, 0.5, -1.0],
                            color: [0., 0., 1., 0.],
                        },
                        Vertex {
                            position: [-0.5, -0.5, -1.0],
                            color: [0., 1., 0., 0.],
                        },
                        Vertex {
                            position: [0.5, -0.5, -1.0],
                            color: [0., 0., 0., 0.],
                        },
                    ],
                    index_buffer: vec![0, 1, 2, 1, 3, 2],
                    clipping: [[-1., 1.], [1., 1.]],
                    draw_mode: pengui::core::DrawMode::TriangleFan,
                    texture: None,
                    uniforms: vec![
                        Uniform::Mat4(to_array(&perspective)),
                        Uniform::Mat4(to_array(&view)),
                        Uniform::Mat4(to_array(&model)),
                    ],
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
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        if key == glutin::event::VirtualKeyCode::Q {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
				}
                _ => (),
			},
			Event::DeviceEvent { event, .. } => match event {
				DeviceEvent::MouseMotion { delta: (dx, dy) } => {
					view_angle_y -= dx as f32 / 800.;
					view_angle_x += dy as f32 / 800.;
				},
				_ => ()
			},
            _ => (),
        }
    });
}
