use pengui::backend::glium::GliumBackend;

mod setup;
use setup::main_window::MainWindow;

mod mesh;
mod meshes;
mod scene;
use scene::Scene;
mod ui;

fn main() {
    let (mut main_window, event_loop, display) = MainWindow::new();
    let backend = GliumBackend::new(display);
    let mut scene = Scene::new(backend);

    event_loop.run(move |event, _, control_flow| {
        main_window.handle_events(&event, control_flow);
        scene.handle_events(&event, &main_window);

        let delta_t = main_window.get_delta_time();
        if delta_t < setup::main_window::MAX_FRAME_DELAY_NS {
            return;
        }
        let time = main_window.new_frame_time();

        scene.ui.ui();
        scene.render(time);

        main_window.end_frame(control_flow);
    });
}
