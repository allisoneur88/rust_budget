use uuid::Uuid;

use crate::{
    Account, Assignment, Budget, Category, Currency, Payee, SuperCategory, SuperTransaction,
    Transaction, User, util::error::AppResult,
};

pub trait UserRepository {
    fn list(&self) -> AppResult<Vec<User>>;
    fn get(&self, id: Uuid) -> AppResult<Option<User>>;
    fn save(&self, user: &User) -> AppResult<()>;
    fn delete(&self, id: Uuid) -> AppResult<()>;
}

pub trait BudgetRepository {
    fn list(&self, user_id: Uuid) -> AppResult<Vec<Budget>>;
    fn get(&self, id: Uuid) -> AppResult<Option<Budget>>;
    fn save(&self, budget: &Budget) -> AppResult<()>;
    fn delete(&self, id: Uuid) -> AppResult<()>;
}

pub trait AccountRepository {
    fn list(&self, budget_id: Uuid) -> AppResult<Vec<Account>>;
    fn get(&self, id: Uuid) -> AppResult<Option<Account>>;
    fn save(&self, account: &Account) -> AppResult<()>;
    fn delete(&self, id: Uuid) -> AppResult<()>;
}

pub trait SuperCategoryRepository {
    fn list(&self, budget_id: Uuid) -> AppResult<Vec<SuperCategory>>;
    fn get(&self, id: Uuid) -> AppResult<Option<SuperCategory>>;
    fn save(&self, super_category: &SuperCategory) -> AppResult<()>;
    fn delete(&self, id: Uuid) -> AppResult<()>;
}

pub trait CategoryRepository {
    fn list(&self, super_category_id: Uuid) -> AppResult<Vec<Category>>;
    fn get(&self, id: Uuid) -> AppResult<Option<Category>>;
    fn save(&self, category: &Category) -> AppResult<()>;
    fn delete(&self, id: Uuid) -> AppResult<()>;
}

pub trait AssignmentRepository {
    fn list(&self, category_id: Uuid) -> AppResult<Vec<Assignment>>;
    fn get(&self, id: Uuid) -> AppResult<Option<Assignment>>;
    fn save(&self, assignment: &Assignment) -> AppResult<()>;
    fn delete(&self, id: Uuid) -> AppResult<()>;
}

pub trait SuperTransactionRepository {
    fn list(&self, budget_id: Uuid) -> AppResult<Vec<SuperTransaction>>;
    fn get(&self, id: Uuid) -> AppResult<Option<SuperTransaction>>;
    fn save(&self, super_transaction: &SuperTransaction) -> AppResult<()>;
    fn delete(&self, id: Uuid) -> AppResult<()>;
}

pub trait TransactionRepository {
    fn list(&self, super_transaction_id: Uuid) -> AppResult<Vec<Transaction>>;
    fn get(&self, id: Uuid) -> AppResult<Option<Transaction>>;
    fn save(&self, transaction: &Transaction) -> AppResult<()>;
    fn delete(&self, id: Uuid) -> AppResult<()>;
}

pub trait PayeeRepository {
    fn list(&self, budget_id: Uuid) -> AppResult<Vec<Payee>>;
    fn get(&self, id: Uuid) -> AppResult<Option<Payee>>;
    fn save(&self, payee: &Payee) -> AppResult<()>;
    fn delete(&self, id: Uuid) -> AppResult<()>;
}

pub trait CurrencyRepository {
    fn list(&self, budget_id: Uuid) -> AppResult<Vec<Currency>>;
    fn get(&self, id: Uuid) -> AppResult<Option<Currency>>;
    fn save(&self, currency: &Currency) -> AppResult<()>;
    fn delete(&self, id: Uuid) -> AppResult<()>;
}
