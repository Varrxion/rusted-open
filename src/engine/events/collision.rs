use crate::engine::graphics::util::master_graphics_list::MasterGraphicsList;

#[derive(Debug, PartialEq)]
pub struct CollisionEvent {
    pub object_id_1: u64,
    pub object_id_2: u64,
}


pub fn check_collisions(master_graphics_list: &MasterGraphicsList, object_id: u64) -> Vec<CollisionEvent> {
    let mut collision_events = Vec::new(); // Vector to hold collision events

    if let Some(object_1) = master_graphics_list.get_object(object_id) {
        let object_1_read = object_1.read().unwrap(); // Access the object through RwLock

        // Iterate over all objects in the MasterGraphicsList
        let all_objects = master_graphics_list.get_objects(); // Get the read-only reference
        for (id, object_2) in all_objects.read().unwrap().iter() {
            // Skip the object being checked against itself
            if *id == object_id {
                continue;
            }

            let object_2_read = object_2.read().unwrap(); // Lock for reading

            // Check for collision
            if object_1_read.is_colliding(&object_2_read) {
                // Create a CollisionEvent and push it into the vector
                collision_events.push(CollisionEvent {
                    object_id_1: object_id,
                    object_id_2: *id,
                });
            }
        }
    } else {
        println!("No object found with ID: {}", object_id);
    }

    collision_events // Return the vector of collision events
}
