use std::sync::{Arc, RwLock};

use glfw::{Action, Context, Key};
use nalgebra::{Matrix4, Vector3};

use crate::engine::graphics;

use super::{events::movement::{self, move_object}, graphics::{assets::base::graphics_object::Generic2DGraphicsObject, util::{master_clock, master_graphics_list::MasterGraphicsList, master_id_generator::MasterIdGenerator}}, state::State};

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

        let window_width = 1280.0;
        let window_height = 720.0;

        // Create a windowed mode window and its OpenGL context
        let (mut window, events) = glfw
            .create_window(window_width as u32, window_height as u32, "Rusted-OpenGL", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        // Set up the projection matrix once
        let projection_matrix = Self::calculate_projection_matrix(window_width, window_height);

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

    fn calculate_projection_matrix(width: f32, height: f32) -> Matrix4<f32> {
        let aspect_ratio = width / height;
        Matrix4::new_orthographic(-1.0, 1.0, -1.0 / aspect_ratio, 1.0 / aspect_ratio, -1.0, 1.0)
    }

    pub fn run_event_loop(&mut self) {  
        // Create the state to manage keys and other state
        let mut state = State::new();

        let newsquare = {
            let basesquare = graphics::assets::square::Square::new();
            Arc::new(RwLock::new(Generic2DGraphicsObject::new(self.master_id_generator.generate_id(), basesquare.get_vertex_data(),basesquare.get_shader_program())))
        };
    
        let newsquareid = self.master_graphics_list.add_object(newsquare);

        // Retrieve the window size
        let (mut previouswidth, mut previousheight) = self.window.get_size(); //keeping this until GLFW update. rough solution
        
        while !self.window.should_close() {
            // Update the clock
            self.master_clock.update();
    
            self.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&self.events) {
                match event {
                    /* //GLFW NEEDS UPDATED, must be some problem with this version 0.50
                    glfw::WindowEvent::Size(width, height) => {
                        // Update the projection matrix on resize
                        self.projection_matrix = Self::calculate_projection_matrix(width as f32, height as f32);
                        println!("MATRIX RECALCED");
                        
                    },
                    */
                    _ => {
                        state.handle_window_event(event); // Handle other window events
                    }
                }
            }
    
            // Retrieve the square from the master graphics list
            let square = self.master_graphics_list.get_object(newsquareid).expect("Object not found");

            let delta_time = self.master_clock.get_delta_time();

            // Apply movement based on active keys
            let speed = 2.0;
            if state.is_key_pressed(Key::W) {
                move_object(square.clone(), Vector3::new(0.0, 1.0, 0.0), speed, delta_time);
            }
            if state.is_key_pressed(Key::S) {
                move_object(square.clone(), Vector3::new(0.0, -1.0, 0.0), speed, delta_time);
            }
            if state.is_key_pressed(Key::A) {
                move_object(square.clone(), Vector3::new(-1.0, 0.0, 0.0), speed, delta_time);
            }
            if state.is_key_pressed(Key::D) {
                move_object(square.clone(), Vector3::new(1.0, 0.0, 0.0), speed, delta_time);
            }

            // Retrieve the window size
            let (width, height) = self.window.get_size();
            if(width!=previouswidth || height!=previousheight) {
                self.projection_matrix = Self::calculate_projection_matrix(width as f32, height as f32);
                println!("MATRIX RECALCED");
                previousheight = height;
                previouswidth = width;
            }

            // Render here
            unsafe {
                gl::ClearColor(0.2, 0.3, 0.3, 1.0); // Set background color
                gl::Clear(gl::COLOR_BUFFER_BIT);    // Clear the screen
            }
    
            // Draw the square
            self.master_graphics_list.draw_all(&self.projection_matrix);
    
            // Swap buffers
            self.window.swap_buffers();
        }
        self.master_graphics_list.remove_all();
    }
}