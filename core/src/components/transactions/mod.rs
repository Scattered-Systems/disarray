/*
   Appellation: transactions <module>
   Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
       ... Summary ...
*/
pub use self::transaction::Transaction;

mod transaction;

pub type Transactions = Vec<Transaction>;
