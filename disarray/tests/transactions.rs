#[cfg(test)]
mod tests {
    use disarray::transactions::Transaction;

    #[test]
    fn test_transaction_default() {
        let a = Transaction::default();
        let b = a.clone();
        assert_eq!(a, b)
    }
}
