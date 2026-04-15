use wgpu::Limits;

mod app;

fn main() {
    let limits = wgpu::Limits {
        max_storage_buffers_per_shader_stage: 8,
        ..Limits::downlevel_defaults()
    };

    dioxus_native::launch_cfg(app::app, vec![], vec![Box::new(limits)]);
    // dioxus_native::launch(app::app)
}
