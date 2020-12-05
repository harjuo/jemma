#![deny(warnings)]
use crate::Operation::{Delete, Get, Head, Post};
use std::rc::Rc;

pub type Path = Vec<Rc<String>>;
pub type ActionResult = Result<Action, String>;

#[derive(Debug)]
pub enum Operation {
    Get,
    Head,
    Delete,
    Post,
}

#[derive(Debug)]
pub struct Action {
    pub op: Operation,
    pub path: Path,
}

/// Parses a GET, HEAD, DELETE or POST HTTP request and returns
/// the operation and path to the requested resource. The path
/// is represented by a ordered list of path fragments.
pub fn get_operation(input: &str) -> ActionResult {
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
                    _ => None,
                };
            }
            PATH => {
                path = fragment
                    .1
                    .split('/')
                    .map(|s| Rc::new(s.to_string()))
                    .collect();
            }
            PROTO => match fragment.1 {
                "HTTP/1.1" => (),
                "HTTP/2" => (),
                _ => return Err("invalid protocol".to_string()),
            },
            _ => {
                return Err("ill-formed query: too many arguments".to_string());
            }
        }
    }

    match op {
        None => Err("invalid operation".to_string()),
        Some(op) => Ok(Action { op, path }),
    }
}

#[test]
fn test1() {
    assert!(get_operation("GET /foo/bar/baz HTTP/1.1").is_ok());
    assert!(get_operation("HEAD /foo/bar/baz HTTP/1.1").is_ok());
    assert!(get_operation("POST /foo/bar/baz HTTP/1.1").is_ok());
    assert!(get_operation("DELETE /foo/bar/baz HTTP/1.1").is_ok());
    assert!(get_operation("ERROR wrong HTTP").is_err());
    assert!(get_operation("GET /foo/bar/baz HTTP/1.1 boo").is_err());
}
