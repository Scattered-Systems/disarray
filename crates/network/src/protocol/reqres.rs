/*
    Appellation: reqres <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MainnetRequest(pub(crate) String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MainnetResponse(pub(crate) Vec<u8>);


