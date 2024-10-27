use crate::engine::graphics::util::master_graphics_list::MasterGraphicsList;

pub fn check_collisions(master_graphics_list: &MasterGraphicsList, object_id: u64) {
    if let Some(object_1) = master_graphics_list.get_object(object_id) {
        let object_1_read = object_1.read().unwrap(); // Access the object through RwLock

        // Iterate over all objects in the MasterGraphicsList
        let all_objects = master_graphics_list.read_only_objects(); // Get the read-only reference
        for (id, object_2) in all_objects.read().unwrap().iter() {
            // Skip the object being checked against itself
            if *id == object_id {
                continue;
            }

            let object_2_read = object_2.read().unwrap(); // Lock for writing to modify if needed
            
            // Check AABB collision
            if object_1_read.is_colliding_aabb(&object_2_read) {
                println!("AABB Collision detected with object ID: {}", id);
            } else {
                //println!("No AABB collision with object ID: {}", id);
            }

            // Check Circle collision
            if object_1_read.is_colliding_circle(&object_2_read) {
                println!("Circle Collision detected with object ID: {}", id);
            } else {
                //println!("No Circle collision with object ID: {}", id);
            }
        }
    } else {
        println!("No object found with ID: {}", object_id);
    }
}
