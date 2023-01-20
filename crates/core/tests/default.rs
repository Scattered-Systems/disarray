#[cfg(test)]

#[test]
fn lib_compiles() {
    let f = |i: usize| i + 1;
    assert_eq!(f(10), 11);
    assert_ne!(f(10), 9)
}

