#[cfg(feature = "network")]
#[cfg(test)]
mod tests {
    use disarray::network::{nodes::*, peers::*, NoiseKeys};

    #[test]
    fn test_default_node() {
        let a = Node::default();
        let b = a.clone();
        assert_eq!(&a.address, &b.address);
    }

    #[test]
    fn test_peer_default() {
        let nk = NoiseKeys::new();
        let a = Peer::default();
        let b = Peer::new(a.id.clone(), a.keypair.clone());
        assert!(a.id == b.id);
        assert!(a.authenticate(nk).is_ok())
    }
}
