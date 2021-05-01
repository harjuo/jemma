#![deny(warnings)]
use decipher::get_operation;
use decipher::ActionResult;
use decipher::Operation::{Delete, Get, Post};
use ephemeral::PathTree;
use futures::executor::block_on;
use std::{
    rc::Rc,
    sync::{Arc, RwLock},
};

struct Storage {
    storage: PathTree<String, bool>,
}

impl Storage {
    fn new() -> Storage {
        Storage {
            storage: PathTree::new(),
        }
    }

    fn act(&mut self, op: ActionResult) {
        match op {
            Ok(action) => match action.op {
                Get => {
                    let result = self.storage.get(&action.path);
                    println!("{:?}: {:?}", action.path, result);
                }
                Post => {
                    self.storage.insert(&action.path, Rc::new(true));
                }
                Delete => self.storage.clear(&action.path),
                _ => (),
            },
            Err(e) => println!("{}", e),
        }
    }
}

async fn do_operation(op: &str, storage: Arc<RwLock<Storage>>) {
    (*storage.write().expect("can't get lock")).act(get_operation(op));
}

/// Demonstration of HTTP parsing and storage functions.
/// POST method sets a true value at a path.
/// GET method displays the value at a path.
async fn async_operations() {
    let storage = Arc::new(RwLock::new(Storage::new()));
    let post = do_operation("POST /foo/bar/baz HTTP/1.1", storage.clone());
    let get = do_operation("GET /foo/bar/baz HTTP/1.1", storage.clone());
    let get_none = do_operation("GET /foo/bar/none HTTP/1.1", storage);
    futures::join!(post, get, get_none);
}

fn main() {
    block_on(async_operations());
}
