// use wgpu::Limits;
use winit::{monitor::Fullscreen, window::WindowAttributes};

mod app;

fn main() {
    // let limits = wgpu::Limits {
    //     max_storage_buffers_per_shader_stage: 8,
    //     ..Limits::downlevel_defaults()
    // };

    let window_options = WindowAttributes::default().with_decorations(false)
        .with_fullscreen(Some(Fullscreen::Borderless(None)));

    dioxus_native::launch_cfg(app::app, vec![], vec![Box::new(window_options)]);
    // dioxus_native::launch(app::app)
}
