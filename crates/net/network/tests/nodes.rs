#[cfg(test)]
use disarray_network::nodes::Node;

#[test]
fn test_default_node() {
    let a = Node::default();
    let b = a.clone();
    assert_eq!(&a.address, &b.address);
}
