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

    pub fn get_ref(&self, path: &Vec<K>) -> Option<&PathTree<K,V>> {
        let mut iteratee = self;
        for fragment in path {
            match iteratee.branches.get(&fragment) {
                None => return None,
                Some(path_tree) => {
                    iteratee = &path_tree;
                },
            }
        }
        Some(iteratee)
    }

    pub fn get(&self, path: &Vec<K>) -> Option<V> {
        match self.get_ref(path) {
            None => None,
            Some(node) => node.leaf,
        }
    }

    fn get_leaves(&self, path: &Vec<K>) -> Vec<(Vec<K>,Option<V>)> where K: Copy, V: Copy {
        let mut leaves = Vec::new();
        leaves.push((path.clone(), self.leaf));
        for branch in self.branches.iter() {
            let mut branch_path = path.clone();
            branch_path.push(*branch.0);
            leaves.append(&mut branch.1.get_leaves(&branch_path));
        }
        leaves
    }

    pub fn get_all(&self, path: &Vec<K>) -> Vec<(Vec<K>,Option<V>)> {
        match self.get_ref(path) {
            None => Vec::new(),
            Some(node) => node.get_leaves(path),
        }
    }

    pub fn list_branches(&self) -> Vec<&K> {
        let mut branches = Vec::new();
        for branch in self.branches.iter() {
            branches.push(branch.0);
        }
        branches
    }

    pub fn get_branch(&self, branch_name: &K) -> Option<&Box<PathTree<K,V>>> {
        self.branches.get(branch_name)
    }

    pub fn get_branch_mut(&mut self, branch_name: &K) -> Option<&mut Box<PathTree<K,V>>> {
        self.branches.get_mut(branch_name)
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

        assert_eq!(puu.get_all(&Vec::new()).len(), 6);
        assert_eq!(
            puu.get_all(&vec!["eka", "toka", "vika", "taas"]), 
                [(vec!["eka", "toka", "vika", "taas"], 
                    Some(33))].to_vec());

    }
}
