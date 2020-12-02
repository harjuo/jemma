use std::collections::HashMap;
use std::hash::Hash;

struct Node<K,V> {
    leaf: Option<V>,
    branches: HashMap<K, Box<Node<K,V>>>,
}

impl<K,V> Node<K,V> 
    where V: Copy, K: Eq+Hash {
    fn new() -> Node<K,V> {
        Node{
            leaf: None, 
            branches: HashMap::new()
        }
    }

    fn get(&self, path: Vec<K>) -> &Option<V> {
        let mut iteratee = self;
        for fragment in path {
            match iteratee.branches.get(&fragment) {
                None => return &None,
                Some(node) => {
                    iteratee = &node;
                },
            }
        }
        &iteratee.leaf
    }

    fn insert(&mut self, path: Vec<K>, value: V) -> Option<V> {
        let mut iteratee = self;
        for fragment in path {
            iteratee = iteratee.branches.entry(fragment).or_insert(Box::new(Node::new()));
        }
        let old_value = iteratee.leaf;
        iteratee.leaf = Some(value);
        old_value
    }
}

#[cfg(test)]
mod tests {
    use crate::Node;

    #[test]
    fn insert_get() {
        let mut puu = Node::new();
        let result = puu.insert(vec!["eka", "toka", "vika"], 42);
        assert_eq!(result, None);
        let result = puu.get(vec!("eka", "toka", "vika"));
        assert_eq!(result.unwrap(), 42);
        let result = puu.insert(vec!["eka", "toka", "vika"], 13);
        assert_eq!(result.unwrap(), 42);
        let result = puu.get(vec!("eka", "toka", "vika"));
        assert_eq!(result.unwrap(), 13);
    }
}
