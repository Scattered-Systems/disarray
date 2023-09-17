#[cfg(test)]
use disarray_network::peers::Peer;
use disarray_network::Network;

#[tokio::test]
async fn test_network_init() {
    let peer = Peer::default();
    let net = Network::from(peer);
    assert!(net.setup().await.is_ok())
}
