#[cfg(test)]

#[test]
fn test_compiles() {
    let f = |x: usize| x + 1;
    assert_eq!(f(10), 11)
}