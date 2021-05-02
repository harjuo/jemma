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
    env,
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

// Stores boolean values for resources that are defined by a path
// of strings. A GET operation to "/this/is/test" stores a true
// value to a resource identified by ["this", "is", "test"].
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

async fn listener(storage: Arc<RwLock<Storage>>, port: u16) -> Result<(), Error> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).map_err(Error::IoError)?;
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

// Get port to run on from the sole optional command line argument.
// Failure to parse exits the process.
fn get_port() -> u16 {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].parse::<u64>() {
            Ok(input) => {
                if input > std::u16::MAX.into() {
                    println!("invalid port number: {}", input);
                    std::process::exit(1);
                }
                input as u16
            }
            Err(_) => {
                println!("invalid port: {}", args[1]);
                std::process::exit(1);
            }
        }
    } else {
        8080
    }
}

// Blocks and waits for incoming connections on the port given as a command
// line argument, or the default port. GET, DELETE and POST operations are
// supported on resources pointed to by paths. Stored values are booleans.
fn main() {
    block_on(
        listener(Arc::new(RwLock::new(Storage::new())), get_port()).map(|result| {
            if let Err(e) = result {
                println!("There was failure: {:?}", e);
            }
        }),
    );
}
