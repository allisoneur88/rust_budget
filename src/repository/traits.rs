use uuid::Uuid;

use crate::{
    Account, Assignment, Budget, Category, Currency, Payee, SuperCategory, SuperTransaction,
    Transaction, User, util::error::AppResult,
};

pub trait UserRepository {
    fn list(&self) -> Vec<User>;
    fn get(&self, id: Uuid) -> Option<User>;
    fn save(&mut self, user: User) -> AppResult<()>;
    fn delete(&mut self, id: Uuid) -> AppResult<()>;
}

pub trait BudgetRepository {
    fn list(&self, user: &User) -> Vec<Budget>;
    fn get(&self, user: &User, id: Uuid) -> Option<Budget>;
    fn save(&mut self, budget: Budget) -> AppResult<()>;
    fn delete(&mut self, id: Uuid) -> AppResult<()>;
}

pub trait AccountRepository {
    fn list(&self, budget: &Budget) -> Vec<Account>;
    fn get(&self, budget: &Budget, id: Uuid) -> Option<Account>;
    fn save(&mut self, account: Account) -> AppResult<()>;
    fn delete(&mut self, id: Uuid) -> AppResult<()>;
}

pub trait SuperCategoryRepository {
    fn list(&self, budget: &Budget) -> Vec<SuperCategory>;
    fn get(&self, budget: &Budget, id: Uuid) -> Option<SuperCategory>;
    fn save(&mut self, super_category: SuperCategory) -> AppResult<()>;
    fn delete(&mut self, id: Uuid) -> AppResult<()>;
}

pub trait CategoryRepository {
    fn list(&self) -> Vec<Category>;
    fn get(&self, id: Uuid) -> Option<Category>;
    fn save(&mut self, category: Category) -> AppResult<()>;
    fn delete(&mut self, id: Uuid) -> AppResult<()>;
}

pub trait AssignmentRepository {
    fn list(&self) -> Vec<Assignment>;
    fn get(&self, id: Uuid) -> Option<Assignment>;
    fn save(&mut self, assignment: Assignment) -> AppResult<()>;
    fn delete(&mut self, id: Uuid) -> AppResult<()>;
}

pub trait SuperTransactionRepository {
    fn list(&self, budget: &Budget) -> Vec<SuperTransaction>;
    fn get(&self, budget: &Budget, id: Uuid) -> Option<SuperTransaction>;
    fn save(&mut self, super_transaction: SuperTransaction) -> AppResult<()>;
    fn delete(&mut self, id: Uuid) -> AppResult<()>;
}

pub trait TransactionRepository {
    fn list(&self) -> Vec<Transaction>;
    fn get(&self, id: Uuid) -> Option<Transaction>;
    fn save(&mut self, transaction: Transaction) -> AppResult<()>;
    fn delete(&mut self, id: Uuid) -> AppResult<()>;
}

pub trait PayeeRepository {
    fn list(&self) -> Vec<Payee>;
    fn get(&self, id: Uuid) -> Option<Payee>;
    fn save(&mut self, payee: Payee) -> AppResult<()>;
    fn delete(&mut self, id: Uuid) -> AppResult<()>;
}

pub trait CurrencyRepository {
    fn list(&self) -> Vec<Currency>;
    fn get(&self, id: Uuid) -> Option<Currency>;
    fn save(&mut self, currency: Currency) -> AppResult<()>;
    fn delete(&mut self, id: Uuid) -> AppResult<()>;
}
