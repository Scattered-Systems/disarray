/*
   Appellation: process <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::specs::*;

pub mod simple;

pub(crate) mod specs {
    use async_trait::async_trait;
    use scsys::prelude::AsyncResult;

    #[async_trait]
    pub trait Processable: Clone + Send + Sync {
        async fn spawn(&self) -> AsyncResult;
    }
}
