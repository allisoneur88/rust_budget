use uuid::Uuid;

use super::category::Category;

#[derive(Debug)]
pub struct SuperCategory {
    pub id: Uuid,
    pub name: String,
    pub categories: Option<Vec<Category>>,
}
