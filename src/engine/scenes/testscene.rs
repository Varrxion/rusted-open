use std::sync::{Arc, RwLock};

use nalgebra::Vector3;

use crate::engine::graphics::{self, assets::base::graphics_object::Generic2DGraphicsObject, texture_manager::TextureManager, util::master_id_generator::MasterIdGenerator};

use super::base::scene::Scene;

// TestScene struct that holds a Scene instance
pub struct TestScene {
    scene: Scene,
}

// Later on, scenes will probably be created using json files and loaded into a hashmap to be more modular, but this is just for testing
impl TestScene {
    // Create a new TestScene
    pub fn new() -> Self {
        TestScene {
            scene: Scene::new(),
        }
    }

    pub fn initialize(&mut self, master_id_generator: Arc<RwLock<MasterIdGenerator>>, texture_manager: Arc<RwLock<TextureManager>>) {

        let newsquaretextureid = texture_manager.read().unwrap().get_texture_id("FamiliarBlock");

        let newsquare = {
            let basesquare = graphics::assets::square_shader::SquareShader::new();
            Arc::new(RwLock::new(Generic2DGraphicsObject::new(master_id_generator.write().unwrap().generate_id(), basesquare.get_vertex_data(), basesquare.get_texture_coords(), basesquare.get_shader_program(), Vector3::new(0.3, 0.0, 0.0), 0.0, 1.0, newsquaretextureid, true)))
        };

        let othersquaretextureid = texture_manager.read().unwrap().get_texture_id("BasicCharacterGreen");

        let othersquare = {
            let basesquare = graphics::assets::square_shader::SquareShader::new();
            Arc::new(RwLock::new(Generic2DGraphicsObject::new(master_id_generator.write().unwrap().generate_id(), basesquare.get_vertex_data(), basesquare.get_texture_coords(), basesquare.get_shader_program(), Vector3::new(-0.3, 0.0, 0.0), 0.0, 1.0, othersquaretextureid, true)))
        };

        // Vertex data
        let vertex_data: Vec<f32> = [
            // Positions (x, y)
            0.3,  0.3,   // Top-right
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
            layout(location = 1) in vec2 aTexCoord;

            out vec2 TexCoords; // Pass to the fragment shader

            uniform mat4 model;
            uniform mat4 projection;

            void main() {
                TexCoords = aTexCoord; // Pass the coordinates along
                gl_Position = projection * model * vec4(aPos, 0.0, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 330 core
            out vec4 FragColor;

            in vec2 TexCoords; // Texture coordinates from the vertex shader

            uniform sampler2D texture_sampler;

            void main() {
                FragColor = texture(texture_sampler, TexCoords); // Sample the texture
            }
        "#;

        let customobjecttextureid = texture_manager.read().unwrap().get_texture_id("BasicCharacterRed");

        let customobject = {
            let custom_shader = graphics::assets::custom_shader::CustomShader::new(vertex_data, texture_coords, &vertex_shader_src, &fragment_shader_src);
            Arc::new(RwLock::new(Generic2DGraphicsObject::new(master_id_generator.write().unwrap().generate_id(), custom_shader.get_vertex_data(), custom_shader.get_texture_coords(), custom_shader.get_shader_program(), Vector3::zeros(), 0.0, 1.0, customobjecttextureid, true)))
        };

        self.add_object(newsquare);
        self.add_object(othersquare);
        self.add_object(customobject);
    }

    // Add an object to the TestScene
    pub fn add_object(&mut self, obj: Arc<RwLock<Generic2DGraphicsObject>>) {
        self.scene.add_object(obj);
    }

    // Get the list of objects in the TestScene
    pub fn get_objects(&self) -> &Vec<Arc<RwLock<Generic2DGraphicsObject>>> {
        self.scene.get_objects()
    }

    pub fn get_scene(&self) -> &Scene {
        &self.scene
    }

}
