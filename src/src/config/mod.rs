use macroquad::{
    miniquad::conf::{LinuxBackend, LinuxX11Gl, Platform},
    window::Conf,
};

/// Initializes the costum config for macroquad to use
pub fn init_config() -> Conf {
    Conf {
        window_title: String::from("smart-road"),
        window_width: 1800,
        window_height: 1000,
        high_dpi: false,
        fullscreen: false,
        sample_count: 1,
        window_resizable: true,
        platform: Platform {
            linux_x11_gl: LinuxX11Gl::GLXWithEGLFallback,
            swap_interval: None,
            linux_backend: LinuxBackend::X11Only,
            framebuffer_alpha: false,
        },
        icon: None,
    }
}
