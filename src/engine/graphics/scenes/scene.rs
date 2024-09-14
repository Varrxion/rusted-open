pub struct Scene {
    objects: Vec<Box<dyn GraphicsObject>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, obj: Box<dyn GraphicsObject>) {
        self.objects.push(obj);
    }

    pub fn update(&mut self, delta_time: f32) {
        for obj in self.objects.iter_mut() {
            obj.update(delta_time);
        }
    }

    pub fn draw(&self) {
        for obj in self.objects.iter() {
            obj.draw();
        }
    }
}
