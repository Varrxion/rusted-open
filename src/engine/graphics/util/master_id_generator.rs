pub struct MasterIdGenerator {
    current_id: u64,
}

impl MasterIdGenerator {
    // the first ID is always 1
    pub fn new() -> Self {
        MasterIdGenerator {
            current_id: 1,
        }
    }

    fn increment_id(&mut self) {
        self.current_id +=1;
    }

    // Generate and return the next ID
    pub fn generate_id(&mut self) -> u64 {
        let new_id = self.current_id;
        self.increment_id();
        new_id
    }
}
