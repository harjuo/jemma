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

const DEBUG: bool = false;

impl Storage {
    fn new() -> Storage {
        Storage {
            storage: PathTree::new(),
        }
    }

    fn act(&mut self, op: ActionResult, mut stream: TcpStream) {
        let stream_error_msg = "cannot write into stream";
        match op {
            Ok(action) => match action.op {
                Get => {
                    let true_reply = b"HTTP/1.1 200 OK\n\n1";
                    let false_reply = b"HTTP/1.1 200 OK\n\n0";
                    let result = self.storage.get(&action.path);
                    let reply = match result.clone() {
                        Some(val) => {
                            if *val {
                                true_reply
                            } else {
                                false_reply
                            }
                        }
                        None => false_reply,
                    };
                    if DEBUG {
                        println!("{:?}: {:?}", action.path, result);
                    }
                    stream
                        .write(reply)
                        .map_err(Error::IoError)
                        .expect(stream_error_msg);
                }
                Post => {
                    if DEBUG {
                        println!("{:?} set", action.path);
                    }
                    self.storage.insert(&action.path, Arc::new(true));
                    stream
                        .write(b"HTTP/1.1 200 OK\n")
                        .map_err(Error::IoError)
                        .expect(stream_error_msg);
                }
                Delete => {
                    self.storage.clear(&action.path);
                    stream
                        .write(b"HTTP/1.1 200 OK\n")
                        .map_err(Error::IoError)
                        .expect(stream_error_msg);
                }
                _ => {
                    stream
                        .write(b"HTTP/1.1 400 Bad Request\n")
                        .map_err(Error::IoError)
                        .expect(stream_error_msg);
                }
            },
            Err(e) => {
                if DEBUG {
                    println!("{}", e);
                }
                stream
                    .write(b"HTTP/1.1 404 Not Found\n")
                    .map_err(Error::IoError)
                    .expect(stream_error_msg);
            }
        }
    }
}

async fn do_operation(op: String, storage: Arc<RwLock<Storage>>, stream: TcpStream) {
    if DEBUG {
        println!("{}", op);
    }
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
    let runner = ThreadPool::new().expect("failed to create thread pool");
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
