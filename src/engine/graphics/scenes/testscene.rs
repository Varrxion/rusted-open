use std::sync::{Arc, RwLock};

use nalgebra::Vector3;

use crate::engine::graphics::{self, assets::base::graphics_object::Generic2DGraphicsObject, util::master_id_generator::{self, MasterIdGenerator}};

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
            let basesquare = graphics::assets::square::Square::new();
            Arc::new(RwLock::new(Generic2DGraphicsObject::new(master_id_generator.write().unwrap().generate_id(), basesquare.get_vertex_data(),basesquare.get_shader_program(), Vector3::new(0.3, 0.0, 0.0), 0.0, 1.0)))
        };

        let othersquare = {
            let basesquare = graphics::assets::square::Square::new();
            Arc::new(RwLock::new(Generic2DGraphicsObject::new(master_id_generator.write().unwrap().generate_id(), basesquare.get_vertex_data(),basesquare.get_shader_program(), Vector3::new(-0.3, 0.0, 0.0), 0.0, 1.0)))
        };

        self.add_object(newsquare);
        self.add_object(othersquare);
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
