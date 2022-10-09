#[cfg(test)]
mod tests {
    use disarray_core::transactions::Transaction;

    #[test]
    fn test_transaction() {
        let a = Transaction::default();
        let b = Transaction::new(None, Vec::new());
        assert_ne!(a, b)
    }
}
