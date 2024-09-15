use gl::types::GLuint;
use crate::engine::graphics::compile::create_shader_program;

pub struct Square {
    vertex_data: Vec<f32>,
    shader_program: GLuint,
}


impl Square {
    pub fn new() -> Self {
        let vertex_data = vec![
            // Dimensions for a square
            -0.1,  0.1,  // Top-left
             0.1,  0.1,  // Top-right
             0.1, -0.1,  // Bottom-right
            -0.1, -0.1,  // Bottom-left
        ];

        let vertex_shader_src = r#"
            #version 330 core
            layout(location = 0) in vec2 aPos;
            uniform mat4 model;
            uniform mat4 projection;
            void main() {
                gl_Position = projection * model * vec4(aPos, 0.0, 1.0);
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

        let square = Square {
            vertex_data,
            shader_program,
        };
        square
    }

    // Returns a copy of vertex data
    pub fn get_vertex_data(&self) -> Vec<f32> {
        self.vertex_data.clone() // Clone the data to transfer ownership
    }

    // Getter for shader_program
    pub fn get_shader_program(&self) -> GLuint {
        self.shader_program
    }
}