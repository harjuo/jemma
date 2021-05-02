# jemma

Storing and fetching boolean values of hierarchically arranged data in ephemeral storage is jemma's sole purpose.

# Usage

Run jemma with `cargo run`. Depending on your platform's restrictions you may need to run it with sudo to let it access port 80.
The program will respond to GET, POST and DELETE requests. Data at the path of the request will be returned, stored or deleted depending on the request. Delete on a root node will delete all the nodes under it as well. The process run until it encounters a critical error, or is interrupted by the user.

# Examples

Run `cargo run` in one shell and the following commands in another shell:

    % curl http://localhost/this/is/test
    0                                                                                                                  
    % curl -X POST http://localhost/this/is/test
    % curl http://localhost/this/is/test     
    1                                                                                                                   
    % curl -X DELETE http://localhost/this/is/test
    % curl http://localhost/this/is/test          
    0
