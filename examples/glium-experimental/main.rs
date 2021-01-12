use glium::Surface;
use std::cell::RefCell;
use std::f32::consts::PI;
extern crate image;

use pengui::{
    backend::glium::GliumBackend,
    core::{CodeLocation, DrawCommand, Event as pgEvent, Mat4x4, Uniforms, Vertex, WidgetBuilder},
    frontend::glutin::Input,
    loc,
    widget::*,
    Interface,
};

mod setup;
use setup::camera::Camera;
use setup::main_window::MainWindow;

fn main() {
    let (mut main_window, event_loop, display) = MainWindow::new();
    let mut backend = GliumBackend::new(display);

    let mut camera = Camera::new();
    use std::io::Cursor;
    let image = image::load(
        Cursor::new(&include_bytes!("../resources/logo_ensps.png")[..]),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    let ensps_tex = backend.register_texture(image);

    let mut ui = Interface::new();

    let font = backend.get_font(0);
    let text = RefCell::new(String::from(
        "Editable text, japanese characters: 色は匂へど散",
    ));

    event_loop.run(move |event, _, control_flow| {
        let font = font.clone();
        let ensps_tex = ensps_tex;
        main_window.handle_events(&event, control_flow);
        camera.handle_events(&event, main_window.alt_pressed);

        use glium::glutin::event::{Event, WindowEvent};
        match &event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::ReceivedCharacter(c) => match c {
                    '\u{8}' => {
                        text.borrow_mut().pop();
                    }
                    _ if *c != '\u{7f}' => text.borrow_mut().push(*c),
                    _ => {}
                },
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(glium::glutin::event::VirtualKeyCode::D) = input.virtual_keycode {
                        if let glium::glutin::event::ElementState::Released = input.state {
                            if main_window.alt_pressed {
                                backend.switch_debug_rendering();
                            }
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        }

        let text = text.clone();
        let delta_t = main_window.get_delta_time();

        ui.new_frame();

        WindowBuilder::new(move |ui| {
            let text = text.clone();
            let font = font.clone();
            if PaddingBuilder::new(
                (0.2, 0.2),
                Button::new("Clickable button".to_string(), font.clone()),
            )
            .build(loc!(), ui.clone())
            {
                println!("Button inside the padding clicked");
            }
            let frame_number = FrameCounter::new()
                .count_next(delta_t >= setup::main_window::MAX_FRAME_DELAY_NS)
                .build(loc!(), ui.clone());
            if CheckBox::new("A checkbox".to_string(), font.clone()).build(loc!(), ui.clone()) {
                Text::new(
                    format!("Frames since beginning : {}", frame_number),
                    font.clone(),
                )
                .build(loc!(), ui.clone());
            }
            Text::new(text.clone().into_inner(), font.clone()).build(loc!(), ui.clone());
            if Button::new("               ".to_string(), font.clone())
                .color((1., 0., 0., 0.5))
                .color((1., 1., 1., 1.))
                .texture(ensps_tex)
                .build(loc!(), ui.clone())
            {
                println!("Button with texture clicked")
            }
            Text::new("↑ Textured button".to_string(), font).build(loc!(), ui.clone());
        })
        .size((20., 12.))
        .build(loc!(), ui.root.clone());

        ui.end_frame();
        ui.generate_layout();
        if let Event::WindowEvent { event, .. } = event {
            if let Some(event) = Input::from(event) {
                let ray = match event {
                    pgEvent::MouseButtonPressed(_) => {
                        let (x, y) = main_window.mouse_pos;
                        Some(camera.ray_from(x, y))
                    }
                    _ => None,
                };
                ui.register_event(event, ray.as_ref());
            }
        }

        let delta_t = main_window.get_delta_time();
        if delta_t < setup::main_window::MAX_FRAME_DELAY_NS {
            return;
        }
        let time = main_window.new_frame_time();

        let mut target = backend.new_frame();
        let blue = (1. + f32::sin(time + PI)) / 2.;
        let red = (1. + f32::sin(time)) / 2.;
        target.clear_color_and_depth((red, 0.0, blue, 1.0), 1.0);

        let (width, height) = target.get_dimensions();
        camera.set_dimensions(width, height);

        let cube_vertices: Vec<Vertex> = vec![
            // Front face
            Vertex {
                position: [-0.5, 0.5, -0.5],
                color: [1., 0., 0., 0.],
                tex_uv: [0., 0.],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                color: [0., 0., 1., 0.],
                tex_uv: [0., 0.],
            },
            Vertex {
                position: [-0.5, -0.5, -0.5],
                color: [0., 1., 0., 0.],
                tex_uv: [0., 0.],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                color: [0., 0., 0., 0.],
                tex_uv: [0., 0.],
            },
            // Right face
            Vertex {
                position: [0.5, -0.5, 0.5],
                color: [1., 1., 0., 0.],
                tex_uv: [0., 0.],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                color: [1., 0., 1., 0.],
                tex_uv: [0., 0.],
            },
            // Left face
            Vertex {
                position: [-0.5, -0.5, 0.5],
                color: [0., 1., 1., 0.],
                tex_uv: [0., 0.],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                color: [1., 1., 1., 0.],
                tex_uv: [0., 0.],
            },
        ];

        let cube_indices: Vec<u32> = vec![
            0, 1, 2, 1, 3, 2, 1, 5, 3, 5, 4, 3, 6, 7, 0, 0, 2, 6, 2, 3, 6, 3, 4, 6,
        ];

        backend
            .draw_command(
                &mut target,
                camera.perspective_view_matrix(),
                Mat4x4::identity(),
                &DrawCommand {
                    vertex_buffer: cube_vertices,
                    index_buffer: cube_indices,
                    draw_mode: pengui::core::DrawMode::Triangles,
                    uniforms: Uniforms::new(),
                },
            )
            .unwrap();
        let list = ui.draw(/*Point3::new(0., 0., 0.), (1., 1.)*/);
        backend
            .draw_list(
                &mut target,
                camera.perspective_view_matrix(),
                Mat4x4::identity(),
                &list,
            )
            .expect("error while rendering ui");

        target.finish().unwrap();

        main_window.end_frame(control_flow);
    });
}
