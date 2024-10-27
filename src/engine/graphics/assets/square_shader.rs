use gl::types::GLuint;
use crate::engine::graphics::compile::create_shader_program;

pub struct SquareShader {
    vertex_data: Vec<f32>,
    texture_coords: Vec<f32>,
    shader_program: GLuint,
}


impl SquareShader {
    pub fn new() -> Self {
        let vertex_data: Vec<f32> = [
            // Positions (x, y)
            0.1,  0.1,   // Top-right
            0.1, -0.1,   // Bottom-right
            -0.1, -0.1,   // Bottom-left
            -0.1,  0.1,   // Top-left
        ].to_vec();

        // Texture coordinates
        let texture_coords: Vec<f32> = [
            // Texture coordinates (u, v)
            1.0, 0.0,   // Top-right
            1.0, 1.0,   // Bottom-right
            0.0, 1.0,   // Bottom-left
            0.0, 0.0,   // Top-left
        ].to_vec();

        let vertex_shader_src = r#"
            #version 330 core
            layout(location = 0) in vec2 aPos;
            layout(location = 1) in vec2 aTexCoord; // Add this line to receive texture coordinates
            out vec2 TexCoord; // Add this line to output texture coordinates
            uniform mat4 model;
            uniform mat4 projection;

            void main() {
                gl_Position = projection * model * vec4(aPos, 0.0, 1.0);
                TexCoord = aTexCoord; // Pass the texture coordinates to the fragment shader
            }
        "#;

        let fragment_shader_src = r#"
            #version 330 core
            out vec4 FragColor;
            in vec2 TexCoord; // Add this line to receive texture coordinates
            uniform sampler2D texture1; // Declare the texture sampler

            void main() {
                FragColor = texture(texture1, TexCoord); // Sample the texture
            }
        "#;

        let shader_program = create_shader_program(vertex_shader_src, fragment_shader_src);

        let square_shader = SquareShader {
            vertex_data,
            texture_coords,
            shader_program,
        };
        square_shader
    }

    // Returns a copy of vertex data
    pub fn get_vertex_data(&self) -> Vec<f32> {
        self.vertex_data.clone() // Clone the data to transfer ownership
    }

    // Returns a copy of texture coords
    pub fn get_texture_coords(&self) -> Vec<f32> {
        self.texture_coords.clone() // Clone the data to transfer ownership
    }

    // Getter for shader_program
    pub fn get_shader_program(&self) -> GLuint {
        self.shader_program
    }
}