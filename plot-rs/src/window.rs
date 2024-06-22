use std::{ffi::c_void, process::Stdio};

use glfw::{GlfwReceiver, WindowEvent};

use crate::core::glDebugCallback;

static mut GLInstanceInitalized: bool = false;

// #[derive(typed_builder::TypedBuilder)]
pub struct Window {
    name: String,
    width: u32,
    height: u32,
    pub m_glfw: glfw::Glfw,
    pub m_window: glfw::PWindow,
    pub m_receiver: GlfwReceiver<(f64, WindowEvent)>,
}

impl Window {
    pub fn new(
        window_name: Option<&str>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<Self, &'static str> {
        let window_name = window_name.unwrap_or("Cycloid");
        let width = width.unwrap_or(1024).min(2 * 1024).max(128);
        let height = height.unwrap_or(1024).min(2 * 1024).max(128);
        unsafe {
            if GLInstanceInitalized == false {
                GLInstanceInitalized = true;
                let glfw = super::core::glfwInit(window_name, width as u32, height as u32);
                return Ok(Self {
                    name: window_name.to_string(),
                    width: width,
                    height: height,
                    m_glfw: glfw.0,
                    m_window: glfw.1,
                    m_receiver: glfw.2,
                });
            } else {
                println!("Error, GLFW is already initalized!");
                return Err("Already Initialized");
            }
        };
    }
    pub fn set_polling(
        &mut self,
        key_poll: bool,
        cursor_poll: bool,
        mouse_poll: bool,
    ) -> &mut Self {
        self.m_window.set_key_polling(key_poll);
        self.m_window.set_cursor_pos_polling(cursor_poll);
        self.m_window.set_mouse_button_polling(mouse_poll);
        self
    }
    pub fn set_view_port(&mut self, x: i32, y: i32, width: i32, height: i32) -> &mut Self {
        unsafe {
            gl::Viewport(x, y, width, height);
        }
        self
    }
    pub fn set_debug_callback(&mut self, enable: bool) -> &mut Self {
        unsafe {
            if enable == true {
                gl::Enable(gl::DEBUG_OUTPUT);
            } else {
                gl::Disable(gl::DEBUG_OUTPUT);
            }
            gl::DebugMessageCallback(Some(glDebugCallback), 0 as _);
        }
        self
    }
}
impl Default for Window {
    fn default() -> Self {
        let mut window = Self::new(None, None, None).unwrap();
        window
            .set_debug_callback(true)
            .set_polling(true, true, true)
            .set_view_port(0, 0, 1024, 1024);
        window
    }
}
