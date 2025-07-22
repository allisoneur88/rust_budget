use super::category::Category;

#[derive(Debug)]
pub struct SuperCategory {
    pub id: u64,
    pub name: String,
    pub categories: Vec<Category>,
}
