/*
   Appellation: process <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use async_trait::async_trait;
use scsys::prelude::BoxResult;

#[async_trait]
pub trait Processable: Clone + Send + Sync + 'static {
    async fn spawn(&self) -> BoxResult;
}
