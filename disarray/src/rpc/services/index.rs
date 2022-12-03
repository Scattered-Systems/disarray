/*
    Appellation: index <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/

#[tarpc::service]
pub trait PingPongService {
    async fn ping();
    async fn pong();
}
