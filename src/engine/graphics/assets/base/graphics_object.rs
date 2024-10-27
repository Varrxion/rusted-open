use gl::types::GLuint;
use nalgebra::{Matrix4, Vector3};
use std::{ffi::CString, sync::{Arc, RwLock}};
use super::{vao::VAO, vbo::VBO};

pub struct Generic2DGraphicsObject {
    id: u64,
    vertex_data: Vec<f32>,
    texture_coords: Vec<f32>,
    vao: Arc<RwLock<VAO>>,
    position_vbo: Arc<VBO>, // VBO for positions
    tex_vbo: Arc<VBO>, // VBO for texture coordinates
    shader_program: GLuint,
    position: nalgebra::Vector3<f32>,
    rotation: f32,
    scale: f32,
    model_matrix: Matrix4<f32>,
    has_collision: bool,
}

impl Clone for Generic2DGraphicsObject {
    fn clone(&self) -> Self {
        Generic2DGraphicsObject {
            id: self.id,
            vertex_data: self.vertex_data.clone(),
            texture_coords: self.texture_coords.clone(),
            vao: Arc::clone(&self.vao),
            position_vbo: Arc::clone(&self.position_vbo),
            tex_vbo: Arc::clone(&self.tex_vbo),
            shader_program: self.shader_program,
            position: self.position,
            rotation: self.rotation,
            scale: self.scale,
            model_matrix: self.model_matrix,
            has_collision: self.has_collision,
        }
    }
}

impl Generic2DGraphicsObject {
    const FULL_ROTATION: f32 = 2.0 * std::f32::consts::PI; // 360 degrees in radians

    pub fn new(
        id: u64,
        vertex_data: Vec<f32>,
        texture_coords: Vec<f32>,
        shader_program: GLuint,
        position: Vector3<f32>,
        rotation: f32,
        scale: f32,
        texture_id: Option<GLuint>, // Accept texture ID as an argument
        has_collision: bool,
    ) -> Self {
        let mut object = Self {
            id,
            vertex_data,
            texture_coords,
            vao: Arc::new(RwLock::new(VAO::new())), // Create a new VAO wrapped in RwLock
            position_vbo: Arc::new(VBO::new(&[])), // Placeholder for position VBO
            tex_vbo: Arc::new(VBO::new(&[])), // Placeholder for texture VBO
            shader_program,
            position,
            rotation,
            scale,
            model_matrix: Matrix4::identity(), // Identity matrix for 2D
            has_collision: has_collision,
        };
        object.initialize(texture_id); // Pass texture ID to initialize
        object
    }

    fn initialize(&mut self, texture_id: Option<GLuint>) {
        let mut vao = self.vao.write().unwrap(); // Lock the RwLock for mutable access
        // Bind the VAO
        vao.bind();

        // Initialize the VBOs with vertex data and texture coordinates
        self.position_vbo = Arc::new(VBO::new(&self.vertex_data)); // Initialize position VBO
        self.tex_vbo = Arc::new(VBO::new(&self.texture_coords)); // Initialize texture VBO

        // Setup vertex attributes for the VAO
        vao.setup_vertex_attributes(vec![
            (self.position_vbo.id(), 2, 0), // Position VBO
            (self.tex_vbo.id(), 2, 1),       // Texture coordinate VBO
        ], texture_id); // Pass texture ID dynamically

        // Unbind the VAO
        VAO::unbind();
    }

    // Apply translation, rotation, and scale as a combined transform
    pub fn update_model_matrix(&mut self) {
        let translation_matrix = Matrix4::new_translation(&self.position);
        let rotation_matrix = Matrix4::new_rotation(Vector3::z() * self.rotation);
        let scale_matrix = Matrix4::new_scaling(self.scale);

        self.model_matrix = translation_matrix * rotation_matrix * scale_matrix; // Combine transformations
    }

    pub fn apply_transform(&self, projection_matrix: &Matrix4<f32>) {
        unsafe {
            // Use the shader program
            gl::UseProgram(self.shader_program);

            // Set the projection matrix
            let projection_location = gl::GetUniformLocation(self.shader_program, CString::new("projection").unwrap().as_ptr());
            let projection_array: [f32; 16] = projection_matrix.as_slice().try_into().expect("Matrix conversion failed");
            gl::UniformMatrix4fv(projection_location, 1, gl::FALSE, projection_array.as_ptr());

            // Set the model matrix
            let model_location = gl::GetUniformLocation(self.shader_program, CString::new("model").unwrap().as_ptr());
            let model_array: [f32; 16] = self.model_matrix.as_slice().try_into().expect("Matrix conversion failed");
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, model_array.as_ptr());
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::UseProgram(self.shader_program);
            let vao = self.vao.read().unwrap(); // Lock the RwLock for read access
            vao.bind();
            // Draw elements based on the number of vertices
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, (self.vertex_data.len() / 2) as i32);
            VAO::unbind();
        }
    }

    // Method to calculate width and height based on vertex data
    fn dimensions(&self) -> (f32, f32) {
        let min_x = self.vertex_data.iter()
            .step_by(2) // Take x-coordinates
            .cloned()
            .fold(f32::INFINITY, f32::min);
        
        let max_x = self.vertex_data.iter()
            .step_by(2) // Take x-coordinates
            .cloned()
            .fold(f32::NEG_INFINITY, f32::max);
        
        let min_y = self.vertex_data.iter()
            .skip(1) // Take y-coordinates
            .step_by(2) // Skip every other (x)
            .cloned()
            .fold(f32::INFINITY, f32::min);
        
        let max_y = self.vertex_data.iter()
            .skip(1) // Take y-coordinates
            .step_by(2) // Skip every other (x)
            .cloned()
            .fold(f32::NEG_INFINITY, f32::max);
        
        let width = (max_x - min_x) * self.scale;
        let height = (max_y - min_y) * self.scale;
        
        (width, height)
    }

    pub fn is_colliding_aabb(&self, other: &Generic2DGraphicsObject) -> bool {
        if !self.has_collision || !other.has_collision {
            return false; // Ignore if collision is not enabled
        }

        let (width_self, height_self) = self.dimensions();
        let (width_other, height_other) = other.dimensions();

        let half_width_self = width_self / 2.0;
        let half_height_self = height_self / 2.0;

        let half_width_other = width_other / 2.0;
        let half_height_other = height_other / 2.0;

        let self_min_x = self.position.x - half_width_self;
        let self_max_x = self.position.x + half_width_self;
        let self_min_y = self.position.y - half_height_self;
        let self_max_y = self.position.y + half_height_self;

        let other_min_x = other.position.x - half_width_other;
        let other_max_x = other.position.x + half_width_other;
        let other_min_y = other.position.y - half_height_other;
        let other_max_y = other.position.y + half_height_other;

        self_min_x < other_max_x &&
        self_max_x > other_min_x &&
        self_min_y < other_max_y &&
        self_max_y > other_min_y
    }

    pub fn is_colliding_circle(&self, other: &Generic2DGraphicsObject) -> bool {
        if !self.has_collision || !other.has_collision {
            return false; // Ignore if collision is not enabled
        }
    
        // Calculate the maximum radius of the first object assuming it is centered at (0,0)
        let radius_self = self.vertex_data.iter()
            .enumerate()
            .step_by(2) // Iterate over x-coordinates
            .map(|(i, &x)| {
                let y = self.vertex_data[i + 1]; // Corresponding y-coordinate
                // Calculate distance from (0,0) to the vertex, scaled by the object’s scale
                let distance = ((x.powi(2) + y.powi(2)).sqrt()) * self.scale;
                distance
            })
            .fold(0.0, f32::max); // Maximum distance gives the effective radius
    
        // Calculate the maximum radius of the second object in the same way
        let radius_other = other.vertex_data.iter()
            .enumerate()
            .step_by(2) // Iterate over x-coordinates
            .map(|(i, &x)| {
                let y = other.vertex_data[i + 1]; // Corresponding y-coordinate
                // Calculate distance from (0,0) to the vertex, scaled by the other object’s scale
                let distance = ((x.powi(2) + y.powi(2)).sqrt()) * other.scale;
                distance
            })
            .fold(0.0, f32::max); // Maximum distance gives the effective radius
    
        // Calculate the distance between the centers
        let dx = other.position.x - self.position.x; // Difference in x-coordinates
        let dy = other.position.y - self.position.y; // Difference in y-coordinates
        let distance_squared = dx * dx + dy * dy; // Distance squared between centers
    
        // Calculate the sum of the radii
        let radius_sum = radius_self + radius_other;
    
        // Check for collision using the squared values to avoid sqrt for performance
        distance_squared < (radius_sum * radius_sum) // Collision if distance is less than sum of radii
    }      

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn set_position(&mut self, position: nalgebra::Vector3<f32>) {
        self.position = position;
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation % Self::FULL_ROTATION;
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn get_model_matrix(&self) -> Matrix4<f32> {
        self.model_matrix
    }

    pub fn get_position(&self) -> nalgebra::Vector3<f32> {
        self.position
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    pub fn get_scale(&self) -> f32 {
        self.scale
    }
}
