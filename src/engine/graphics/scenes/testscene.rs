use std::sync::{Arc, RwLock};

use nalgebra::Vector3;

use crate::engine::graphics::{self, assets::{base::graphics_object::Generic2DGraphicsObject, custom_shader}, util::master_id_generator::{self, MasterIdGenerator}};

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

    pub fn initialize(&mut self, master_id_generator: Arc<RwLock<MasterIdGenerator>>) {
        let newsquare = {
            let basesquare = graphics::assets::square_shader::SquareShader::new();
            Arc::new(RwLock::new(Generic2DGraphicsObject::new(master_id_generator.write().unwrap().generate_id(), basesquare.get_vertex_data(),basesquare.get_shader_program(), Vector3::new(0.3, 0.0, 0.0), 0.0, 1.0)))
        };

        let othersquare = {
            let basesquare = graphics::assets::square_shader::SquareShader::new();
            Arc::new(RwLock::new(Generic2DGraphicsObject::new(master_id_generator.write().unwrap().generate_id(), basesquare.get_vertex_data(),basesquare.get_shader_program(), Vector3::new(-0.3, 0.0, 0.0), 0.0, 1.0)))
        };

        let vertex_data = vec![
            // Dimensions for a square
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
                FragColor = vec4(1.0, 0.0, 1.0, 1.0);
            }
        "#;

        let customobject = {
            let custom_shader = graphics::assets::custom_shader::CustomShader::new(vertex_data, &vertex_shader_src, &fragment_shader_src);
            Arc::new(RwLock::new(Generic2DGraphicsObject::new(master_id_generator.write().unwrap().generate_id(), custom_shader.get_vertex_data(), custom_shader.get_shader_program(), Vector3::zeros(), 0.0, 1.0)))
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
