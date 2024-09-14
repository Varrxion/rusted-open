use std::time::Instant;

use glfw::{Action, Context, Key};

use crate::engine::graphics;
use crate::engine::util::master_clock;

use super::util::master_clock::MasterClock;

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
        .create_window(800, 600, "Rusted-OpenGL", glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();

    // Enable key events
    window.set_key_polling(true);

    // Load OpenGL functions
    graphics::glfw::init();
    

    // Create a Square instance
    let mut square = graphics::assets::square::Square::new();

    let mut master_clock = master_clock::MasterClock::new();

    // Main render loop
    while !window.should_close() {
        //update the clock
        master_clock.update();
        
        // Poll events
        glfw.poll_events();

        // Handle window events
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        // Update the square's position
        let movement_speed = 0.2; // Speed of movement
        let mut posvector = square.get_position();
        posvector.x += movement_speed * master_clock.get_delta_time();
        println!("{}", posvector.x);
        square.set_position(posvector); // Accumulate translation over time
        square.set_rotation(master_clock.get_delta_time()*3.0);
        square.update_model_matrix();

        // Apply the transformation to the shader
        square.apply_transform();

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