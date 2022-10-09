/*
    Appellation: builder <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::transport::TransportConfig;
use scsys::core::BoxResult;
///
#[derive(Clone)]
pub struct TransportBuilder;

impl TransportBuilder {
    pub fn configure(self) -> BoxResult<TransportConfig<'static>> {
        Ok(TransportConfig::default())
    }
}
