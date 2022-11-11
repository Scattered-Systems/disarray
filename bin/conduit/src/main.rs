/*
    Appellation: Conduit <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
#[doc(inline)]
pub use self::{actors::*, components::*, core::*, data::*};

pub(crate) mod actors;
pub(crate) mod components;
pub(crate) mod core;
pub(crate) mod data;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> scsys::prelude::BoxResult {
    println!("Hello, world!");

    let app = Application::<contexts::Context>::default();
    println!("{}", &app);
    app.run().await?;

    Ok(())
}
