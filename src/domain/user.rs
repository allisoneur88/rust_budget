use super::budget::Budget;

#[derive(Debug)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub password: Option<String>,
    pub budgets: Option<Vec<Budget>>,
}
