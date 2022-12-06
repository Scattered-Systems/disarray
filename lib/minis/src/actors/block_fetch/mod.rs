/*
    Appellation: block_fetch <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub use self::agent::BlockFetch;

pub(crate) mod agent;

pub(crate) mod state {
    pub enum State {}
}
