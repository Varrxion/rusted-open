use std::{collections::HashMap, sync::{Arc, RwLock}};
use nalgebra::Matrix4;

use crate::engine::{graphics::assets::base::graphics_object::Generic2DGraphicsObject, scenes::base::scene::Scene};

pub struct MasterGraphicsList {
    objects: Arc<RwLock<HashMap<u64, Arc<RwLock<Generic2DGraphicsObject>>>>>,
}

impl MasterGraphicsList {
    // Initialize a new MasterGraphicsList
    pub fn new() -> Self {
        MasterGraphicsList {
            objects: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // Add an object to the list and return its ID
    pub fn add_object(&self, obj: Arc<RwLock<Generic2DGraphicsObject>>) -> u64 {
        let id = obj.read().unwrap().get_id();
        let mut objects = self.objects.write().unwrap();
        objects.insert(id, obj);
        id
    }

    // Add multiple objects from a Scene to the MasterGraphicsList
    pub fn load_scene(&self, scene: &Scene) {
        for obj in scene.get_objects().iter() {
            // Clone the object first to get a new instance
            let cloned_obj = obj.read().unwrap().clone(); // Clone the actual object
            
            // Wrap the cloned object in Arc and RwLock
            let arc_obj = Arc::new(RwLock::new(cloned_obj));
            
            // Add to the master list
            self.add_object(arc_obj);
        }
    }

    // Get an object by ID
    pub fn get_object(&self, id: u64) -> Option<Arc<RwLock<Generic2DGraphicsObject>>> {
        let objects = self.objects.read().unwrap();
        objects.get(&id).cloned()
    }

    // Returns a pointer to the entire object list
    pub fn read_only_objects(&self) -> Arc<RwLock<HashMap<u64, Arc<RwLock<Generic2DGraphicsObject>>>>> {
        Arc::clone(&self.objects) // Return a clone of the Arc to allow shared access
    }

    // Draw all objects in the list
    pub fn draw_all(&self, projection_matrix: &Matrix4<f32>) {
        let objects = self.objects.read().unwrap(); // Lock for reading the list
        for obj in objects.values() {
            if let Ok(mut obj) = obj.write() { // Lock each object for writing (to update model matrix)
                obj.update_model_matrix(); // Update the model matrix first
                obj.apply_transform(projection_matrix); // Apply the projection matrix
                obj.draw(); // Now draw the object
            }
        }
    }
    
    // Remove an object by ID
    pub fn remove_object(&self, id: u64) {
        let mut objects = self.objects.write().unwrap();
        objects.remove(&id);
    }

    // Remove all objects from the list
    pub fn remove_all(&self) {
        let mut objects = self.objects.write().unwrap();
        objects.clear();
    }
}
