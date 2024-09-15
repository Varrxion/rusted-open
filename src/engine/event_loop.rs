use std::sync::{Arc, RwLock};

use glfw::{Action, Context, Key};
use nalgebra::Matrix4;

use crate::engine::graphics;

use super::graphics::{assets::base::graphics_object::Generic2DGraphicsObject, util::{master_clock, master_graphics_list::MasterGraphicsList, master_id_generator::{self, MasterIdGenerator}}};

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

    let window_width = 1900.0;
    let window_height = 800.0;

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw
        .create_window(window_width as u32, window_height as u32, "Rusted-OpenGL", glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");

    // Set up the projection matrix once
    let aspect_ratio = window_width / window_height; // Adjust based on window size
    let projection_matrix = Matrix4::new_orthographic(-1.0, 1.0, -1.0 / aspect_ratio, 1.0 / aspect_ratio, -1.0, 1.0);

    // Make the window's context current
    window.make_current();

    // Enable key events
    window.set_key_polling(true);

    // Load OpenGL functions
    graphics::glfw::init();

    // Initialize the master graphics list
    let mut master_graphics_list = MasterGraphicsList::new();

    let mut master_id_generator = MasterIdGenerator::new();

    let mut master_clock = master_clock::MasterClock::new();

    run_event_loop(&mut glfw, &mut window, &events, &projection_matrix, &mut master_graphics_list, &mut master_id_generator, &mut master_clock)
}

fn run_event_loop(
    glfw: &mut glfw::Glfw,
    window: &mut glfw::Window,
    events: &std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    projection_matrix: &Matrix4<f32>,
    master_graphics_list: &mut MasterGraphicsList,
    master_id_generator: &mut MasterIdGenerator,
    master_clock: &mut master_clock::MasterClock
) {
    let full_rotation = 2.0 * std::f32::consts::PI; // 360 degrees in radians

    let newsquare = {
        let basesquare = graphics::assets::square::Square::new();
        Arc::new(RwLock::new(Generic2DGraphicsObject::new(master_id_generator.generate_id(), basesquare.get_vertex_data(),basesquare.get_shader_program())))
    };

    let newsquareid = master_graphics_list.add_object(newsquare);
    
    while !window.should_close() {
        // Update the clock
        master_clock.update();
            
        // Poll events
        glfw.poll_events();

        // Handle window events
        for (_, event) in glfw::flush_messages(events) {
            handle_window_event(window, event);
        }

        // Retrieve the square from the master graphics list
        let square = master_graphics_list.get_object(newsquareid).expect("Object not found");

        // Lock for read access
        let mut square = square.write().unwrap();

        // Update the square's position
        let movement_speed = 0.2; // Speed of movement
        let mut posvector = square.get_position();
        posvector.x += movement_speed * master_clock.get_delta_time();
        posvector.y += movement_speed/3.0 * master_clock.get_delta_time();
        if posvector.x >= 1.0 {
            posvector.x -= 2.0;
        }
        if posvector.y >= 0.5 {
            posvector.y -= 1.0;
        }
        println!("Position X Factor: {}", posvector.x);
        println!("Position Y Factor: {}", posvector.y);
        square.set_position(posvector); // Accumulate translation over time

        let rotation_speed = 2.3;
        let mut rotation_factor = square.get_rotation();
        rotation_factor += rotation_speed * master_clock.get_delta_time();
        if rotation_factor >= full_rotation {
            rotation_factor -= full_rotation;
        }
        println!("Rotation Factor: {}", rotation_factor);
        square.set_rotation(rotation_factor);

        square.update_model_matrix();

        // Apply the transformation to the shader
        square.apply_transform(projection_matrix);

        // Render here
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0); // Set background color
            gl::Clear(gl::COLOR_BUFFER_BIT);    // Clear the screen
        }

        std::mem::drop(square);

        // Draw the square
        master_graphics_list.draw_all();

        // Swap buffers
        window.swap_buffers();
    }
    master_graphics_list.remove_all();
}