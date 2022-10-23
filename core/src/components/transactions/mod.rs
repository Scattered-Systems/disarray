/*
   Appellation: transactions <module>
   Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
       ... Summary ...
*/
pub use self::{signed::SignedTransaction, transaction::Transaction};

mod signed;
mod transaction;

pub type Transactions = Vec<Transaction>;

mod traits {
    pub struct TxId<I64>(I64);

    impl<I64> TxId<I64> {
        pub fn new(data: I64) -> Self {
            Self(data)
        }
    }

    pub trait TransactionSpec {
        fn id(&self) -> TxId<i64>;
    }
}
