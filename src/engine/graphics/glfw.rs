use std::ffi::CString;

fn load_gl_symbols() {
    gl::load_with(|s| {
        let c_str = CString::new(s).unwrap();
        unsafe { glfw::ffi::glfwGetProcAddress(c_str.as_ptr()) as *const _ }
    });
}

pub fn init() {
    // Load OpenGL functions
    load_gl_symbols();
}