/*
    Appellation: Conduit <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
#[doc(inline)]
pub use self::{application::*, settings::*};

pub mod agents;
pub mod cli;
pub mod contexts;
pub mod rpc;
pub mod sessions;
pub mod states;

pub(crate) mod application;
pub(crate) mod settings;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> scsys::prelude::BoxResult {
    println!("Hello, world!");

    let app = Application::<contexts::Context>::default();
    println!("{}", &app);
    app.with_tracing()?.run().await?;

    Ok(())
}
