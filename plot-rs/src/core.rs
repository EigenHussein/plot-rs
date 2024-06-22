use std::ffi::c_void;

use glfw::{GlfwReceiver, WindowEvent};
use glfw::Context;
pub extern "system"  fn glDebugCallback(
    source: u32,
    type_t: u32,
    id: u32,
    severity: u32,
    length: i32,
    message: *const i8,
    user_param: *mut c_void,
) {
    if severity == gl::DEBUG_SEVERITY_HIGH {
        // Handle high-severity messages (e.g., errors)
        println!("OpenGL Error (High Severity): {:?}", message);
    } else if severity == gl::DEBUG_SEVERITY_MEDIUM {
        // Handle medium-severity messages (e.g., warnings)
        println!("OpenGL Error (MED Severity): {:?}", message);
    } else {
        // Handle low-severity messages (e.g., performance notifications)
        println!("OpenGL Error (LOW Severity): {:?}", message);
    }
}

pub fn glfwInit(
    window_name: &str,
    width: u32,
    height: u32,
) -> (glfw::Glfw, glfw::PWindow, GlfwReceiver<(f64, WindowEvent)>) {
    let mut glfw_ = glfw::init_no_callbacks().unwrap(); //glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    unsafe {
        // glfwWindowHint(glfw::WindowHint::, hint)

        glfw::ffi::glfwWindowHint(glfw::ffi::CONTEXT_VERSION_MAJOR, 4);
        glfw::ffi::glfwWindowHint(glfw::ffi::CONTEXT_VERSION_MINOR, 3);
        glfw::ffi::glfwWindowHint(glfw::ffi::OPENGL_PROFILE, glfw::ffi::OPENGL_CORE_PROFILE);
        glfw::ffi::glfwWindowHint(glfw::ffi::SAMPLES, 16);
    }

    let (mut window, events) = glfw_
        .create_window(
            width,
            height,
            window_name,
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    glfw_.set_swap_interval(glfw::SwapInterval::Sync(2));
    gl::load_with(|f| glfw_.get_proc_address_raw(f));
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        // glBlendFuncSeparate(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA, GL_ONE, GL_ONE);
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::MULTISAMPLE);
    }

    println!("{}", gl::ClearColor::is_loaded());
    return (glfw_, window, events);
}


pub fn file2string(filename: &str) -> String {
    return std::fs::read_to_string(filename).unwrap();
}
pub fn read_both(vertex: &str, fragment: &str) -> (String, String) {
    let vertex_source = std::fs::read_to_string(vertex).unwrap();
    let fragment_source = std::fs::read_to_string(fragment).unwrap();

    return (vertex_source.to_string(), fragment_source.to_string());
}