use std::sync::{Arc, RwLock};
use crate::engine::graphics::assets::base::graphics_object::Generic2DGraphicsObject;

pub struct MasterGraphicsList {
    objects: RwLock<Vec<Arc<RwLock<Generic2DGraphicsObject>>>>,
}

impl MasterGraphicsList {
    // Initialize a new MasterGraphicsList
    pub fn new() -> Self {
        MasterGraphicsList {
            objects: RwLock::new(Vec::new()),
        }
    }

    // Add an object to the list
    pub fn add_object(&self, obj: Arc<RwLock<Generic2DGraphicsObject>>) {
        let mut objects = self.objects.write().unwrap();
        objects.push(obj);
    }

    // Get an object by index
    pub fn get_object(&self, index: usize) -> Option<Arc<RwLock<Generic2DGraphicsObject>>> {
        let objects = self.objects.read().unwrap();
        objects.get(index).cloned()
    }

    // Draw all objects in the list
    pub fn draw_all(&self) {
        let objects = self.objects.read().unwrap(); // Lock for reading the list
        for obj in objects.iter() {
            if let Ok(obj) = obj.read() { // Lock each object only while drawing
                obj.draw();
            }
        }
    }
    

    // Remove all objects from the list
    pub fn remove_all(&self) {
        let mut objects = self.objects.write().unwrap();
        objects.clear();
    }
}
