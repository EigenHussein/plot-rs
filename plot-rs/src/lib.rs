pub mod core;
pub mod window;
pub mod shader_builder;
pub mod shaders;
pub mod coordinate_mapping;

pub mod scene;
pub mod layers;
pub mod camera;

pub mod buffers;
pub mod shapes;
pub mod coord2d;

pub mod textures;
pub mod texts;
// pub mod opengl-core;

#[cfg(target_os = "windows")]
pub static FONT_PATH: &'static str = "C:\\Users\\Haloc\\Dev\\Rust\\plot-rs\\cambriaz.ttf";
#[cfg(target_os = "linux")]
pub static FONT_PATH: &'static str = "/home/hussein/Dev/Rust/plotters/cambriaz.ttf";
