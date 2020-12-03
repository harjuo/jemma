use std::collections::HashMap;
use std::hash::Hash;

/// PathTree represents a tree structure where every value is identified
/// by a list of keys called a path.
pub struct PathTree<K,V> {
    leaf: Option<V>,
    branches: HashMap<K,Box<PathTree<K,V>>>,
}

impl<K,V> PathTree<K,V> 
    where V: Copy, K: Copy+Eq+Hash {
    pub fn new() -> PathTree<K,V> {
        PathTree{
            leaf: None, 
            branches: HashMap::new()
        }
    }

    pub fn get(&self, path: &Vec<K>) -> &Option<V> {
        let mut iteratee = self;
        for fragment in path {
            match iteratee.branches.get(&fragment) {
                None => return &None,
                Some(path_tree) => {
                    iteratee = &path_tree;
                },
            }
        }
        &iteratee.leaf
    }

    pub fn insert(&mut self, path: &Vec<K>, value: V) -> Option<V> {
        let mut iteratee = self;
        for fragment in path {
            iteratee = iteratee.branches.entry(*fragment).or_insert(Box::new(PathTree::new()));
        }
        let old_value = iteratee.leaf;
        iteratee.leaf = Some(value);
        old_value
    }
}

#[cfg(test)]
mod tests {
    use crate::PathTree;

    #[test]
    fn insert_get() {
        let mut puu = PathTree::new();
        let result = puu.insert(&vec!["eka", "toka", "vika"], 42);
        assert_eq!(result, None);
        let result = puu.get(&vec!["eka", "toka", "vika"]);
        assert_eq!(result.unwrap(), 42);
        let result = puu.insert(&vec!["eka", "toka", "vika"], 13);
        assert_eq!(result.unwrap(), 42);
        let result = puu.get(&vec!["eka", "toka", "vika"]);
        assert_eq!(result.unwrap(), 13);
        let result = puu.insert(&vec!["eka", "kolmas"], 12);
        assert_eq!(result, None);
        let result = puu.get(&vec!["eka", "kolmas"]);
        assert_eq!(result.unwrap(), 12);
        let result = puu.insert(&vec!["eka", "toka", "vika", "taas"], 33);
        assert_eq!(result, None);
        let result = puu.get(&vec!["eka", "toka", "vika", "taas"]);
        assert_eq!(result.unwrap(), 33);
    }
}
