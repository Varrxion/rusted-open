use gl::types::{GLfloat, GLint, GLuint};
use nalgebra::{Matrix4, Vector3};
use std::ffi::CString;

use super::{vao::VAO, vbo::VBO};

pub struct Generic2DGraphicsObject {
    id: u64,
    vertex_data: Vec<f32>,
    vao: VAO,
    vbo: VBO,
    shader_program: GLuint,
    position: nalgebra::Vector3<f32>,
    rotation: f32,
    scale: f32,
    model_matrix: Matrix4<f32>,
}

impl Generic2DGraphicsObject {
    pub fn new(id: u64, vertex_data: Vec<f32>, shader_program: GLuint) -> Self {
        let mut object = Self {
            id,
            vertex_data,
            vao: VAO::new(), // Create a new VAO
            vbo: VBO::new(&[]), // Placeholder, will initialize in `initialize` method
            shader_program,
            position: nalgebra::Vector3::zeros(),
            rotation: 0.0,
            scale: 1.0,
            model_matrix: Matrix4::identity(), // Identity matrix for 2D
        };
        object.initialize();
        object
    }

    fn initialize(&mut self) {
        unsafe {
            // Bind the VAO
            self.vao.bind();

            // Initialize the VBO with vertex data
            self.vbo = VBO::new(&self.vertex_data);
            self.vbo.bind();

            // Specify the layout of the vertex data for 2D
            gl::VertexAttribPointer(
                0, // Attribute index
                2, // Number of components per vertex attribute (2 for 2D positions)
                gl::FLOAT, // Data type of each component
                gl::FALSE, // Whether the data should be normalized
                2 * std::mem::size_of::<GLfloat>() as GLint, // Stride (spacing between consecutive attributes)
                std::ptr::null(), // Pointer to the start of the data
            );
            gl::EnableVertexAttribArray(0); // Enable the vertex attribute array for index 0

            // Unbind the VAO
            VAO::unbind();

            // Unbind the VBO
            VBO::unbind();
        }
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
            self.vao.bind();
            // Draw elements based on the number of vertices
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, (self.vertex_data.len() / 2) as i32);
            VAO::unbind();
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn set_position(&mut self, position: nalgebra::Vector3<f32>) {
        self.position = position;
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
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
