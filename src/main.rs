#![deny(warnings)]
use decipher::get_operation;
use decipher::ActionResult;
use decipher::Operation::{Delete, Get, Post};
use ephemeral::PathTree;
use futures::executor::{block_on, ThreadPool};
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
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
                    self.storage.insert(&action.path, Arc::new(true));
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

async fn do_operation_owned(op: String, storage: Arc<RwLock<Storage>>) {
    do_operation(&op, storage).await;
}

fn handle_client(
    mut stream: TcpStream,
    runner: ThreadPool,
    storage: Arc<RwLock<Storage>>,
) -> std::io::Result<()> {
    let mut buffa = [0; 1280];
    let read_bytes = stream.read(&mut buffa)?;
    println!("read {} bytes", read_bytes);
    let request = std::str::from_utf8(&buffa).expect("can't convert to string");
    let first_line = request.lines().next().expect("empty request").to_owned();
    println!("{}", first_line);
    runner.spawn_ok(do_operation_owned(first_line, storage));
    Ok(())
}

async fn connection_listener_inner(storage: Arc<RwLock<Storage>>) -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;
    let runner = ThreadPool::new().expect("failed to create thread pool");
    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?, runner.clone(), storage.clone())?;
    }

    Ok(())
}

async fn connection_listener(storage: Arc<RwLock<Storage>>) {
    connection_listener_inner(storage)
        .await
        .expect("listening failed");
}

/// Demonstration of HTTP parsing and storage functions.
/// POST method sets a true value at a path.
/// GET method displays the value at a path.
async fn async_operations() {
    let storage = Arc::new(RwLock::new(Storage::new()));
    let listener = connection_listener(storage.clone());
    let post = do_operation("POST /foo/bar/baz HTTP/1.1", storage.clone());
    let get = do_operation("GET /foo/bar/baz HTTP/1.1", storage.clone());
    let get_none = do_operation("GET /foo/bar/none HTTP/1.1", storage);
    futures::join!(listener, post, get, get_none);
}

fn main() {
    block_on(async_operations());
}
