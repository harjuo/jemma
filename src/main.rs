#![deny(warnings)]
use decipher::get_operation;
use decipher::ActionResult;
use decipher::Operation::{Delete, Get, Post};
use ephemeral::PathTree;
use futures::{
    executor::{block_on, ThreadPool},
    prelude::*,
};
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

async fn do_operation(op: String, storage: Arc<RwLock<Storage>>) {
    (*storage.write().expect("can't get lock")).act(get_operation(&op));
}

fn handle_client(
    mut stream: TcpStream,
    runner: ThreadPool,
    storage: Arc<RwLock<Storage>>,
) -> std::io::Result<()> {
    const BUF_SIZE: usize = 1280;
    let mut buffa = [0; BUF_SIZE];
    if stream.read(&mut buffa)? > 0 {
        runner.spawn_ok(do_operation(
            std::str::from_utf8(&buffa)
                .expect("can't convert to string")
                .lines()
                .next()
                .expect("empty request")
                .to_owned(),
            storage,
        ));
    }
    Ok(())
}

async fn listener(storage: Arc<RwLock<Storage>>) -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;
    let runner = ThreadPool::new().expect("failed to create thread pool");
    for request in listener.incoming() {
        handle_client(request?, runner.clone(), storage.clone())?;
    }
    Ok(())
}

fn main() {
    block_on(
        listener(Arc::new(RwLock::new(Storage::new())))
            .map(|r| r.expect("could not create connection listener")),
    );
}
