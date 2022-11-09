/*
    Appellation: core <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State<T: ToString> {
    pub message: T
}

impl<T: ToString> State<T> {
    pub fn new(message: T) -> Self {
        Self { message }
    }
}

impl<T: Default + ToString> Default for State<T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}
