use uuid::Uuid;

pub struct IdGenerator {}

impl IdGenerator {
    pub fn new_id() -> Uuid {
        Uuid::new_v4()
    }
}
