#[cfg(test)]
mod tests {
    use disarray_core::transactions::{generate_random_transaction, Transaction};

    #[test]
    fn test_transaction_default() {
        let a = Transaction::default();
        let b = generate_random_transaction();
        assert_ne!(a, b)
    }
}
