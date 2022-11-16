/*
   Appellation: consentual <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description: ... summary ...
*/

pub trait Consentual: ToString {
    type Proof: ToString;

    fn agreement(&self) -> bool;
    
}