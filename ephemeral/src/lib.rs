use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

/// PathTree represents a structure where every value is identified
/// by a path. The path consists of a vector of elements called
/// fragments. Multiple values in a PathTree can be retrieved
/// or deleted by referring to a position and applying an operation
/// to all branches at the position.
pub struct PathTree<K, V> {
    leaf: Option<Rc<V>>,
    branches: HashMap<Rc<K>, Box<PathTree<K, V>>>,
}

impl<K, V> PathTree<K, V>
where
    K: Eq + Hash,
{
    /// Constructs a new empty PathTree
    pub fn new() -> PathTree<K, V> {
        PathTree {
            leaf: None,
            branches: HashMap::new(),
        }
    }

    /// Gets an immutable reference to a position at a given path
    pub fn get_ref(&self, path: &Vec<Rc<K>>) -> Option<&PathTree<K, V>> {
        let mut iteratee = self;
        for fragment in path {
            match iteratee.branches.get(fragment) {
                None => return None,
                Some(path_tree) => {
                    iteratee = &path_tree;
                }
            }
        }
        Some(iteratee)
    }

    /// Gets a reference to a value at given path
    pub fn get(&self, path: &Vec<Rc<K>>) -> Option<Rc<V>> where V: Clone {
        match self.get_ref(path) {
            None => None,
            Some(node) => node.leaf.clone(),
        }
    }

    // Utility function used by get_all
    fn get_leaves(&self, path: &Vec<Rc<K>>) -> Vec<(Vec<Rc<K>>, Option<Rc<V>>)> {
        let mut leaves = Vec::new();
        leaves.push((path.clone(), self.leaf.clone()));
        for branch in self.branches.iter() {
            let mut branch_path = path.clone();
            branch_path.push(branch.0.clone());
            leaves.append(&mut branch.1.get_leaves(&branch_path).clone());
        }
        leaves
    }

    /// Gets all values in the branches at the path including the position
    /// at the path itself.
    pub fn get_all(&self, path: &Vec<Rc<K>>) -> Vec<(Vec<Rc<K>>, Option<Rc<V>>)> {
        match self.get_ref(path) {
            None => Vec::new(),
            Some(node) => node.get_leaves(path),
        }
    }

    /// Lists all branches at the position
    pub fn list_branches(&self) -> Vec<Rc<K>> {
        let mut branches: Vec<Rc<K>> = Vec::new();
        for branch in self.branches.iter() {
            branches.push(branch.0.clone());
        }
        branches
    }

    /// Gets an immutable reference to a branch at the position identified by
    /// a path fragment
    pub fn get_branch(&self, branch_name: Rc<K>) -> Option<&Box<PathTree<K, V>>> {
        self.branches.get(&branch_name)
    }

    /// Gets a mutable reference to a branch at the position identified by
    /// a path fragment.
    pub fn get_branch_mut(&mut self, branch_name: Rc<K>) -> Option<&mut Box<PathTree<K, V>>> {
        self.branches.get_mut(&branch_name)
    }

    /// Inserts a value at a given path. If a value already exists
    /// at the position it is returned and replaced by the new value.
    pub fn insert(&mut self, path: &Vec<Rc<K>>, value: Rc<V>) -> Option<Rc<V>> {
        let mut iteratee = self;
        for fragment in path {
            iteratee = iteratee
                .branches
                .entry(fragment.clone())
                .or_insert(Box::new(PathTree::new()));
        }
        let old_value = iteratee.leaf.clone();
        iteratee.leaf = Some(value.clone());
        old_value
    }

    /// Clears a value at a given path if a position exists at the path
    pub fn clear(&mut self, path: &Vec<Rc<K>>) {
        let mut iteratee = self;
        for fragment in path {
            match iteratee.branches.get_mut(fragment) {
                None => return,
                Some(path_tree) => {
                    iteratee = path_tree;
                }
            }
        }
        iteratee.leaf = None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_get() {
        let mut puu = PathTree::new();
        let eka = Rc::new("eka");
        let eka_2 = Rc::new("eka");
        let eka_3 = Rc::new("eka");
        let toka = Rc::new("toka");
        let vika = Rc::new("vika");
        let kolmas = Rc::new("kolmas");
        let taas = Rc::new("taas");
        let n42 = Rc::new(42);
        let n12 = Rc::new(12);
        let n13 = Rc::new(13);
        let n33 = Rc::new(33);
        

        let result = puu.insert(&vec![eka.clone(), toka.clone(), vika.clone()], &n42);
        assert_eq!(result, None);
        let result = puu.get(&vec![eka_2.clone(), toka.clone(), vika.clone()]);
        assert_eq!(result.unwrap(), n42);
        let result = puu.insert(&vec![eka_3.clone(), toka.clone(), vika.clone()], &n13);
        assert_eq!(result.unwrap(), n42);
        let result = puu.get(&vec![eka.clone(), toka.clone(), vika.clone()]);
        assert_eq!(result.unwrap(), n13);
        let result = puu.insert(&vec![eka_2.clone(), kolmas.clone()], &n12);
        assert_eq!(result, None);
        let result = puu.get(&vec![eka_3.clone(), kolmas.clone()]);
        assert_eq!(result.unwrap(), n12);
        let result = puu.insert(&vec![eka.clone(), toka.clone(), vika.clone(), taas.clone()], &n33);
        assert_eq!(result, None);
        let result = puu.get(&vec![eka_2.clone(), toka.clone(), vika.clone(), taas.clone()]);
        assert_eq!(result.unwrap(), n33);

        assert_eq!(puu.get_all(&Vec::new()).len(), 6);
        assert_eq!(
            puu.get_all(&vec![eka.clone(), toka.clone(), vika.clone(), taas.clone()]),
            [(vec![eka.clone(), toka.clone(), vika.clone(), taas.clone()], Some(n33))].to_vec()
        );
    }
}
