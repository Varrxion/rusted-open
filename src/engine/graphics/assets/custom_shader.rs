use gl::types::GLuint;
use crate::engine::graphics::compile::create_shader_program;

pub struct CustomShader {
    vertex_data: [f32; 8],
    texture_coords: [f32; 8],
    shader_program: GLuint,
}


impl CustomShader {
    pub fn new(vertex_data: [f32; 8], texture_coords: [f32; 8], vertex_shader_src: &str, fragment_shader_src: &str) -> Self {
        let shader_program = create_shader_program(vertex_shader_src, fragment_shader_src);

        let custom_shader = CustomShader {
            vertex_data,
            texture_coords,
            shader_program,
        };
        custom_shader
    }

    // Returns a copy of vertex data
    pub fn get_vertex_data(&self) -> [f32; 8] {
        self.vertex_data.clone() // Clone the data to transfer ownership
    }

    // Returns a copy of texture coords
    pub fn get_texture_coords(&self) -> [f32; 8] {
        self.texture_coords.clone() // Clone the data to transfer ownership
    }

    // Getter for shader_program
    pub fn get_shader_program(&self) -> GLuint {
        self.shader_program
    }
}