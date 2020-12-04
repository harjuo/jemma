use decipher::{get_operation};
use decipher::ActionResult;
use decipher::Operation::{Get, Post, Delete};
use ephemeral::PathTree;
use std::rc::Rc;

struct Storage {
    storage: PathTree<String,bool>
}

impl Storage where {
    fn new() -> Storage {
        Storage { storage: PathTree::new() }
    }

    fn act(&mut self, op: ActionResult) {
        match op {
            Ok(action) => {
                match action.op {
                    Get => {
                        let result = self.storage.get(&action.path);
                        println!("{:?}: {:?}", action.path, result);
                    }
                    Post => {
                        self.storage.insert(&action.path, Rc::new(true));
                    }
                    Delete => {
                        self.storage.clear(&action.path)
                    }
                    _ => ()
                }
            }
            Err(e) => println!("{}", e)
        }
    }
}



fn main() {
    let mut storage = Storage::new();
    storage.act(get_operation("POST /foo/bar/baz HTTP/1.1"));
    storage.act(get_operation("GET /foo/bar/baz HTTP/1.1"));
    storage.act(get_operation("GET /foo/bar/none HTTP/1.1"));
}
