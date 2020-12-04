use std::f32::consts::PI;

use glium::Surface;
use nalgebra::*;
extern crate image;

use pengui::{
    backend::glium::GliumBackend,
    core::{CodeLocation, DrawCommand, Uniforms, Vertex, WidgetBuilder},
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

    event_loop.run(move |event, _, control_flow| {
        let ensps_tex = ensps_tex;
        main_window.handle_events(&event, control_flow);
        camera.handle_events(&event);

        let delta_t = main_window.get_delta_time();
        if delta_t < setup::main_window::MAX_FRAME_DELAY_NS {
            return;
        }
        let time = main_window.new_frame_time();

        let mut target = backend.new_frame();
        let blue = (1. + f32::sin(time + PI)) / 2.;
        let red = (1. + f32::sin(time)) / 2.;
        target.clear_color_and_depth((red, 0.0, blue, 1.0), 1.0);

        ui.new_frame();
        WindowBuilder::new(move |ui| {
            PaddingBuilder::new((0.2, 0.2), Button::new("label not displayed".to_string()))
                .build(loc!(), ui.clone());
            Button::new("label not displayed".to_string()).build(loc!(), ui.clone());
            Button::new("label not displayed".to_string())
                .color((1., 0., 0., 0.5))
                .color((1., 1., 1., 1.))
                .texture(ensps_tex)
                .build(loc!(), ui.clone());
        })
        .build(loc!(), ui.root.clone());

        ui.end_frame();
        ui.generate_layout();
        let list = ui.draw(Point3::new(0., 0., 0.), (1., 1.));
        backend
            .draw_list(&mut target, camera.perspective_view_matrix(), &list)
            .expect("error while rendering ui");

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
                &DrawCommand {
                    vertex_buffer: cube_vertices,
                    index_buffer: cube_indices,
                    draw_mode: pengui::core::DrawMode::Triangles,
                    uniforms: Uniforms::new(),
                },
            )
            .unwrap();

        target.finish().unwrap();

        main_window.end_frame(control_flow);
    });
}
