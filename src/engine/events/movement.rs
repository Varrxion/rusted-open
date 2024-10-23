use std::sync::{Arc, RwLock};
use nalgebra::Vector3;
use crate::engine::graphics::assets::base::graphics_object::Generic2DGraphicsObject;

pub fn move_object(object: Arc<RwLock<Generic2DGraphicsObject>>, direction: Vector3<f32>, speed: f32, delta_time: f32) {
    let mut object = object.write().unwrap();
    let mut pos = object.get_position();

    // Apply movement in the given direction
    pos += direction * speed * delta_time;

    // Update the position and model matrix
    object.set_position(pos);
    object.update_model_matrix();
}