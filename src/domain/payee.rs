use uuid::Uuid;

#[derive(Debug)]
pub struct Payee {
    pub id: Uuid,
    pub name: String,
}
