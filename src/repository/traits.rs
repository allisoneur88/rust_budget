use uuid::Uuid;

use crate::{
    Account, Assignment, Budget, Category, Payee, SuperCategory, SuperTransaction, Transaction,
    User,
};

pub trait UserRepository {
    fn list(&self) -> Vec<User>;
    fn get(&self, id: Uuid) -> Option<User>;
    fn save(&mut self, user: User);
    fn delete(&mut self, id: Uuid);
}

pub trait BudgetRepository {
    fn list(&self) -> Vec<Budget>;
    fn get(&self, id: Uuid) -> Option<Budget>;
    fn save(&mut self, budget: Budget);
    fn delete(&mut self, id: Uuid);
}

pub trait AccountRepository {
    fn list(&self) -> Vec<Account>;
    fn get(&self, id: Uuid) -> Option<Account>;
    fn save(&mut self, account: Account);
    fn delete(&mut self, id: Uuid);
}

pub trait SuperCategoryRepository {
    fn list(&self) -> Vec<SuperCategory>;
    fn get(&self, id: Uuid) -> Option<SuperCategory>;
    fn save(&mut self, super_category: SuperCategory);
    fn delete(&mut self, id: Uuid);
}

pub trait CategoryRepository {
    fn list(&self) -> Vec<Category>;
    fn get(&self, id: Uuid) -> Option<Category>;
    fn save(&mut self, category: Category);
    fn delete(&mut self, id: Uuid);
}

pub trait AssignmentRepository {
    fn list(&self) -> Vec<Assignment>;
    fn get(&self, id: Uuid) -> Option<Assignment>;
    fn save(&mut self, assignment: Assignment);
    fn delete(&mut self, id: Uuid);
}

pub trait SuperTransactionRepository {
    fn list(&self) -> Vec<SuperTransaction>;
    fn get(&self, id: Uuid) -> Option<SuperTransaction>;
    fn save(&mut self, super_transaction: SuperTransaction);
    fn delete(&mut self, id: Uuid);
}

pub trait TransactionRepository {
    fn list(&self) -> Vec<Transaction>;
    fn get(&self, id: Uuid) -> Option<Transaction>;
    fn save(&mut self, transaction: Transaction);
    fn delete(&mut self, id: Uuid);
}

pub trait PayeeRepository {
    fn list(&self) -> Vec<Payee>;
    fn get(&self, id: Uuid) -> Option<Payee>;
    fn save(&mut self, payee: Payee);
    fn delete(&mut self, id: Uuid);
}
