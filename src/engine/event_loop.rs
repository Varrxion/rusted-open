use glfw::{Action, Context, Key};

use crate::engine::graphics;

// Handle window events like key presses
fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}

pub fn start() {
    // Initialize GLFW
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw
        .create_window(800, 600, "Hello OpenGL", glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();

    // Enable key events
    window.set_key_polling(true);

    // Load OpenGL functions
    graphics::glfw::init();

    // Create a Square instance
    let square = graphics::assets::square::Square::new();

    // Main render loop
    while !window.should_close() {
        // Poll events
        glfw.poll_events();

        // Handle window events
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        // Render here
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0); // Set background color
            gl::Clear(gl::COLOR_BUFFER_BIT);    // Clear the screen
        }

        // Draw the square
        square.draw();

        // Swap buffers
        window.swap_buffers();
    }
}