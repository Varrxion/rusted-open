use std::sync::{Arc, RwLock};

use glfw::{Action, Context, Key};
use nalgebra::Matrix4;

use crate::engine::graphics;

use super::{events::movement, graphics::{assets::base::graphics_object::Generic2DGraphicsObject, util::{master_clock, master_graphics_list::MasterGraphicsList, master_id_generator::MasterIdGenerator}}};

pub struct EventLoop {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    master_graphics_list: MasterGraphicsList,
    master_id_generator: MasterIdGenerator,
    master_clock: master_clock::MasterClock,
    projection_matrix: Matrix4<f32>,
}

impl EventLoop {
    pub fn new() -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let window_width = 1900.0;
        let window_height = 800.0;

        // Create a windowed mode window and its OpenGL context
        let (mut window, events) = glfw
            .create_window(window_width as u32, window_height as u32, "Rusted-OpenGL", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        // Set up the projection matrix once
        let aspect_ratio = window_width / window_height; // Adjust based on window size
        let projection_matrix = Matrix4::new_orthographic(-1.0, 1.0, -1.0 / aspect_ratio, 1.0 / aspect_ratio, -1.0, 1.0);

        // Make the window's context current
        window.make_current();

        // Enable key events
        window.set_key_polling(true);

        // Load OpenGL functions
        graphics::glfw::load_gl_symbols();

        // Initialize the master graphics list
        let mut master_graphics_list = MasterGraphicsList::new();

        let mut master_id_generator = MasterIdGenerator::new();

        let mut master_clock = master_clock::MasterClock::new();

        Self {
            glfw,
            window,
            events,
            master_graphics_list,
            master_id_generator,
            master_clock,
            projection_matrix,
        }
    }

    // Handle window events like key presses
    fn handle_window_event(&mut self) {
        // Poll events
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true)
                }
                // Retrieve the object by ID (assuming ID is known or stored)
                glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => {
                    // Retrieve the object by ID (assuming ID is known or stored)
                    if let Some(square) = self.master_graphics_list.get_object(1) { // Replace `1` with the actual ID
                        // Call the move_up function
                        movement::move_up(square, 2.0, self.master_clock.get_delta_time());
                    }
                }
                _ => {}
            }
        }
    }

    pub fn run_event_loop(&mut self) {
        let full_rotation = 2.0 * std::f32::consts::PI; // 360 degrees in radians
    
        let newsquare = {
            let basesquare = graphics::assets::square::Square::new();
            Arc::new(RwLock::new(Generic2DGraphicsObject::new(self.master_id_generator.generate_id(), basesquare.get_vertex_data(),basesquare.get_shader_program())))
        };
    
        let newsquareid = self.master_graphics_list.add_object(newsquare);
        
        while !self.window.should_close() {
            // Update the clock
            self.master_clock.update();
    
            self.handle_window_event();
    
            // Retrieve the square from the master graphics list
            let square = self.master_graphics_list.get_object(newsquareid).expect("Object not found");
    
            // Lock for read access
            let mut square = square.write().unwrap();
    
            // Update the square's position
            let movement_speed = 0.2; // Speed of movement
            let mut posvector = square.get_position();
            posvector.x += movement_speed * self.master_clock.get_delta_time();
            posvector.y += movement_speed/3.0 * self.master_clock.get_delta_time();
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
            rotation_factor += rotation_speed * self.master_clock.get_delta_time();
            if rotation_factor >= full_rotation {
                rotation_factor -= full_rotation;
            }
            println!("Rotation Factor: {}", rotation_factor);
            square.set_rotation(rotation_factor);
    
            square.update_model_matrix();
    
            // Apply the transformation to the shader
            square.apply_transform(&self.projection_matrix);
    
            // Render here
            unsafe {
                gl::ClearColor(0.2, 0.3, 0.3, 1.0); // Set background color
                gl::Clear(gl::COLOR_BUFFER_BIT);    // Clear the screen
            }
    
            std::mem::drop(square);
    
            // Draw the square
            self.master_graphics_list.draw_all();
    
            // Swap buffers
            self.window.swap_buffers();
        }
        self.master_graphics_list.remove_all();
    }
}