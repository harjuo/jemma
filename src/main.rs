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

#[derive(Debug)]
enum Error {
    IoError(std::io::Error),
    Other,
}

impl Storage {
    fn new() -> Storage {
        Storage {
            storage: PathTree::new(),
        }
    }

    fn act(&mut self, op: ActionResult, mut stream: TcpStream) {
        let reply = match op {
            Ok(action) => match action.op {
                Get => {
                    let result = self.storage.get(&action.path);
                    let get_value = match result {
                        Some(val) => {
                            if *val {
                                "1"
                            } else {
                                "0"
                            }
                        }
                        None => "0",
                    };
                    "HTTP/1.1 200 OK\n\n".to_owned() + get_value
                }
                Post => {
                    self.storage.insert(&action.path, Arc::new(true));
                    "HTTP/1.1 200 OK\n".to_owned()
                }
                Delete => {
                    self.storage.clear(&action.path);
                    "HTTP/1.1 200 OK\n".to_owned()
                }
                _ => "HTTP/1.1 400 Bad Request\n".to_owned(),
            },
            Err(_) => "HTTP/1.1 404 Not Found\n".to_owned(),
        };
        if let Err(e) = stream.write(reply.as_bytes()) {
            println!("error writing to stream: {:?}", e);
        }
    }
}

async fn do_operation(op: String, storage: Arc<RwLock<Storage>>, stream: TcpStream) {
    (*storage.write().expect("can't get lock")).act(get_operation(&op), stream);
}

fn handle_client(
    mut stream: TcpStream,
    runner: ThreadPool,
    storage: Arc<RwLock<Storage>>,
) -> Result<(), Error> {
    const BUF_SIZE: usize = 1280;
    let mut buffa = [0; BUF_SIZE];
    if stream.read(&mut buffa).map_err(Error::IoError)? > 0 {
        runner.spawn_ok(do_operation(
            std::str::from_utf8(&buffa)
                .map_err(|_| Error::Other)?
                .lines()
                .next()
                .ok_or(Error::Other)?
                .to_owned(),
            storage,
            stream,
        ));
    }
    Ok(())
}

async fn listener(storage: Arc<RwLock<Storage>>) -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:80").map_err(Error::IoError)?;
    let runner = ThreadPool::new().map_err(Error::IoError)?;
    for request in listener.incoming() {
        handle_client(
            request.map_err(Error::IoError)?,
            runner.clone(),
            storage.clone(),
        )?;
    }
    Ok(())
}

fn main() {
    block_on(
        listener(Arc::new(RwLock::new(Storage::new()))).map(|result| {
            if let Err(e) = result {
                println!("There was failure: {:?}", e);
            }
        }),
    );
}
