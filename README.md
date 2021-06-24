# jemma

This repository serves mostly as an exercise for the author in Rust's data structures and asynchronous networking programming. There is a very bare-bones HTTP protocol handler that stores and retrieves hierarchically structures data from an ephemeral storage. Currently only boolean values are supported, but extending the code to handle more complex data mainly would require extending the HTTP parser. The ephemeral storage is able to handle keys and values of arbitrary complexity.

# Requirements

Building jemma requires the standard Rust toolchain that is available at https://rustup.rs.

# Usage

Run jemma with `cargo run`. Optionally, you can give a port number, like `cargo run 5000`. Default port that jemma listens to is 8080.

The program will respond to GET, POST and DELETE requests for any given path. Data at the path of the request will be returned, stored or deleted depending on the request. Delete on a root node will delete all the nodes under it as well.

The process will run until it encounters a critical error, or is interrupted by the user.

# Examples

Run `cargo run` in one shell and the following commands in another shell:

    % curl http://localhost:8080/this/is/test
    0
    % curl -X POST http://localhost:8080/this/is/test
    % curl http://localhost:8080/this/is/test
    1
    % curl -X DELETE http://localhost:8080/this/is/test
    % curl http://localhost:8080/this/is/test
    0
