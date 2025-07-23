use super::budget::Budget;
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password: Option<String>,
    pub budgets: Option<Vec<Budget>>,
}
