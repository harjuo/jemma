use crate::Operation::{Get, Head, Delete, Post};

type Path = Vec<String>;

#[derive(Debug)]
enum Operation {
    Get,
    Head,
    Delete,
    Post,
}

#[derive(Debug)]
struct Action {
    op: Operation,
    path: Path,
}

fn get_operation(input: &str) -> Result<Action, String> {
    const OP: usize = 0;
    const PATH: usize = 1;
    const PROTO: usize = 2;
    let mut op = None;
    let mut path = Vec::new();
    for fragment in input.split_whitespace().enumerate() {
        match fragment.0 {
            OP => {
                op = match fragment.1 {
                    "GET" => Some(Get),
                    "HEAD" => Some(Head),
                    "DELETE" => Some(Delete),
                    "POST" => Some(Post),
                    _ => None
                };
            },
            PATH => {
                path = fragment.1.split('/').map(String::from).collect();
            },
            PROTO => match fragment.1 {
                "HTTP/1.1" => (),
                "HTTP/2" => (),
                _ => return Err("invalid protocol".to_string())
            },
            _ => {
                return Err("ill-formed query: too many arguments".to_string());
            }
        }
    }

    match op {
        None => Err("invalid operation".to_string()),
        Some(op) => Ok(Action{op, path})
    }

}

fn main() {
    let parcel = "GET /foo/bar/baz HTTP/1.1";
    let op = get_operation(parcel);
    println!("{:?}", op);
}
