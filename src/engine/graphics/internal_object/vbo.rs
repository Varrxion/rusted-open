extern crate gl;
use std::mem;
use gl::types::*;

pub struct VBO {
    id: GLuint, // Stores the VBO ID generated by OpenGL
}

impl VBO {
    /// Creates a new Vertex Buffer Object and uploads the provided vertex data.
    pub fn new(data: &[f32]) -> Self {
        let mut vbo: GLuint = 0;

        unsafe {
            // Generate a new buffer
            gl::GenBuffers(1, &mut vbo);

            // Bind the buffer (GL_ARRAY_BUFFER means it is a vertex buffer)
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            // Upload the vertex data to the buffer
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            // Unbind the buffer to avoid accidental modification
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        Self {
            id: vbo,
        }
    }

    /// Returns the VBO ID.
    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for VBO {
    /// Clean up the buffer when it's no longer needed (automatically called by Rust).
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
