use uuid::Uuid;

#[derive(Debug)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
}
