use std::sync::{Arc, RwLock};

use nalgebra::Vector3;
use crate::engine::graphics::assets::base::graphics_object::Generic2DGraphicsObject;

pub fn move_up(square: Arc<RwLock<Generic2DGraphicsObject>>, movement_speed: f32, delta_time: f32) {

    // Get write access to the square
    let mut square = square.write().unwrap();
    // Get the current position
    let mut pos = square.get_position();

    // Move the object upwards
    pos.y += movement_speed * delta_time;

    // Update the position and model matrix
    square.set_position(pos);
    square.update_model_matrix();
}
