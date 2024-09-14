use std::ffi::CString;
use gl::types::{GLint, GLuint, GLfloat};
use nalgebra::{Matrix4, Vector3};
use crate::engine::graphics::vbo::VBO;
use crate::engine::graphics::vao::VAO;
use crate::engine::graphics::compile::create_shader_program;

use super::graphics_object::GraphicsObject;

pub struct Square {
    vertex_data: Vec<f32>,
    vao: VAO,
    vbo: VBO,
    shader_program: GLuint,
    graphics_object: GraphicsObject,
}


impl Square {
    pub fn new() -> Self {
        let vertex_data = vec![
            // Dimensions for a square
            -0.5,  0.5, 0.0,  // Top-left
             0.5,  0.5, 0.0,  // Top-right
             0.5, -0.5, 0.0,  // Bottom-right
            -0.5, -0.5, 0.0,  // Bottom-left
        ];

        let vertex_shader_src = r#"
            #version 330 core
            layout(location = 0) in vec3 aPos;
            uniform mat4 model;
            void main() {
                gl_Position = model * vec4(aPos, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 330 core
            out vec4 FragColor;
            void main() {
                FragColor = vec4(1.0, 0.0, 0.0, 1.0); // Red color
            }
        "#;

        let shader_program = create_shader_program(vertex_shader_src, fragment_shader_src);

        let mut square = Square {
            vertex_data,
            vao: VAO::new(), // Create a new VAO
            vbo: VBO::new(&[]), // Placeholder, will initialize in `initialize` method
            shader_program,
            graphics_object: GraphicsObject::new(),
        };
        square.initialize();
        square
    }

    fn initialize(&mut self) {
        unsafe {
            // Bind the VAO
            self.vao.bind();

            // Initialize the VBO with vertex data
            self.vbo = VBO::new(&self.vertex_data);
            self.vbo.bind();

            // Specify the layout of the vertex data
            gl::VertexAttribPointer(
                0,                             // Attribute index
                3,                             // Number of components per vertex attribute
                gl::FLOAT,                      // Data type of each component
                gl::FALSE,                      // Whether the data should be normalized
                3 * std::mem::size_of::<GLfloat>() as GLint, // Stride (spacing between consecutive attributes)
                std::ptr::null(),               // Pointer to the start of the data
            );
            gl::EnableVertexAttribArray(0);   // Enable the vertex attribute array for index 0

            // Unbind the VAO to avoid accidental modifications
            VAO::unbind();

            // Unbind the VBO (not necessary but can be done for consistency)
            VBO::unbind();
        }
    }

    pub fn update_model_matrix(&mut self) {
        self.graphics_object.update_model_matrix();
    }

    // Send the model matrix to the shader
    pub fn apply_transform(&self) {
        let model_location = unsafe {
            gl::GetUniformLocation(self.shader_program, CString::new("model").unwrap().as_ptr())
        };

        // Convert nalgebra Matrix4 to a flat array of f32
        let model_array: [f32; 16] = self.graphics_object.get_model_matrix().as_slice().try_into().expect("Matrix conversion failed");

        unsafe {
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, model_array.as_ptr());
        }
    }
    

    pub fn draw(&self) {
        unsafe {
            gl::UseProgram(self.shader_program);
            self.vao.bind();
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
            VAO::unbind();
        }
    }

    pub fn get_position(&self) -> Vector3<f32> {
        self.graphics_object.get_position()
    }

    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.graphics_object.set_position(position);
    }

    pub fn get_rotation(&self) -> f32 {
        self.graphics_object.get_rotation()
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.graphics_object.set_rotation(rotation);
    }

    pub fn get_scale(&self) -> f32 {
        self.graphics_object.get_scale()
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.graphics_object.set_scale(scale);
    }
}