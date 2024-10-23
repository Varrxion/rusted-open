use std::{collections::HashMap, sync::{Arc, RwLock}};
use nalgebra::Matrix4;

use crate::engine::graphics::assets::base::graphics_object::Generic2DGraphicsObject;

pub struct MasterGraphicsList {
    objects: RwLock<HashMap<u64, Arc<RwLock<Generic2DGraphicsObject>>>>,
}

impl MasterGraphicsList {
    // Initialize a new MasterGraphicsList
    pub fn new() -> Self {
        MasterGraphicsList {
            objects: RwLock::new(HashMap::new()),
        }
    }

    // Add an object to the list and return its ID
    pub fn add_object(&self, obj: Arc<RwLock<Generic2DGraphicsObject>>) -> u64 {
        let id = obj.read().unwrap().get_id();
        let mut objects = self.objects.write().unwrap();
        objects.insert(id, obj);
        id
    }

    // Get an object by ID
    pub fn get_object(&self, id: u64) -> Option<Arc<RwLock<Generic2DGraphicsObject>>> {
        let objects = self.objects.read().unwrap();
        objects.get(&id).cloned()
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
