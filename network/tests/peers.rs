#[cfg(test)]
mod tests {
    use disarray_network::{peers::*, NoiseKeys};

    #[test]
    fn test_peer_default() {
        let nk = NoiseKeys::new();
        let a = Peer::default();
        let b = Peer::new(a.id, a.keypair.clone());
        assert!(a.id == b.id);
        assert!(a.authenticate(nk).is_ok())
    }
}
