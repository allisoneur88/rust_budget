use crate::{
    Category, IdGenerator,
    domain::assignment::{Assignment, YearMonth},
};

#[derive(Debug)]
pub struct AssignmentService;

impl AssignmentService {
    pub fn new() -> Self {
        Self
    }

    pub fn make_assignment(
        &self,
        amount: f64,
        year_month: YearMonth,
        category: &Category,
    ) -> Assignment {
        Assignment {
            id: IdGenerator::new_id(),
            amount,
            year_month,
            category_id: category.id,
        }
    }

    pub fn update_amount(&self, assigment: &mut Assignment, new_amount: f64) {
        assigment.amount = new_amount;
    }
}
