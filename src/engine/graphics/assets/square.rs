use gl::types::GLint;
use gl::types::GLuint;
use gl::types::GLfloat;
use crate::engine::graphics::vbo::VBO;
use crate::engine::graphics::vao::VAO;
use crate::engine::graphics::compile::create_shader_program;

pub struct Square {
    vertex_data: Vec<f32>,
    vao: VAO,        // Using the VAO struct
    vbo: VBO,        // Using the VBO struct
    shader_program: GLuint,
}

impl Square {
    pub fn new() -> Self {
        let vertex_data = vec![
            // Positions for a square
            -0.5,  0.5, 0.0,  // Top-left
             0.5,  0.5, 0.0,  // Top-right
             0.5, -0.5, 0.0,  // Bottom-right
            -0.5, -0.5, 0.0,  // Bottom-left
        ];

        let vertex_shader_src = r#"
            #version 330 core
            layout(location = 0) in vec3 aPos;
            void main() {
                gl_Position = vec4(aPos, 1.0);
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

    pub fn draw(&self) {
        unsafe {
            gl::UseProgram(self.shader_program);
            self.vao.bind();
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
            VAO::unbind();
        }
    }
}
