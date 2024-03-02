/*
   Appellation: actors <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::rt::procs::Processable;

use async_trait::async_trait;
use scsys::prelude::{AsyncResult, Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SimpleProc;

#[async_trait]
impl Processable for SimpleProc {
    async fn spawn(&self) -> AsyncResult {
        simple_proc().await;
        Ok(())
    }
}

pub async fn simple_proc() {
    let _tmp = Timestamp::default();
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
}
