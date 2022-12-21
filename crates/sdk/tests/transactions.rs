#[cfg(test)]
mod tests {
    use disarray_sdk::ledger::transactions::{Sign, SignedTransaction, Transaction};

    #[test]
    fn test_default_transaction() {
        let a = Transaction::default();
        let b = a.clone();
        assert_eq!(a, b)
    }

    #[test]
    fn test_default_signed_transaction() {
        let signature = Sign::default();
        let transaction = Transaction::default();

        let a = SignedTransaction::new(signature, transaction);
        let b = SignedTransaction::default();
        assert_eq!(a, b)
    }
}
