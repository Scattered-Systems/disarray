/*
    Appellation: builder <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::transport::TransportConfig;

///
#[derive(Clone)]
pub struct TransportBuilder;

impl TransportBuilder {
    pub fn configure(self) -> Result<TransportConfig<'static>, scsys::BoxError> {
        Ok(TransportConfig::default())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize, y: usize| x + y;
        assert_eq!(f(4, 2), 6)
    }
}
