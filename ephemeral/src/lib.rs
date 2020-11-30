use std::collections::HashMap;
use std::sync::Arc;

enum Value {
    Integer(u64),
    SignedInteger(i64),
    Phrase(String),
}

struct Node {
    leaf: Option<Arc<Value>>,
    branches: HashMap<String, Box<Node>>,
}

impl Node {
    fn new() -> Node {
        Node{leaf: None, branches: HashMap::new()}
    }

    fn get(&self, path: Vec<&str>, root: &Node) -> Option<Arc<Value>> {
        let mut iteratee = root;
        for fragment in path {
            match iteratee.branches.get(fragment) {
                None => return None,
                Some(node) => iteratee = &node,
            }
        }
        iteratee.leaf.clone()
    }

    // TODO put & delete

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
