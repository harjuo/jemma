#!/bin/bash

cargo build
target/debug/jemma 5000&
pid=$!

get=$(curl http://localhost:5000/this/is/test)

if [[ $get =~ "0" ]]; then
    echo "GET OK"
else
    echo "GET not OK"
    kill "$pid"
    exit 1
fi

curl -X POST http://localhost:5000/this/is/test

if [[ $? -eq 0 ]]; then
    echo "POST OK"
else
    echo "POST not OK"
    kill "$pid"
    exit 1
fi

get=$(curl http://localhost:5000/this/is/test)

if [[ $get =~ "1" ]]; then
    echo "GET OK"
else
    echo "GET not OK"
    kill "$pid"
    exit 1
fi

curl -X DELETE http://localhost:5000/this/is/test

if [[ $? -eq 0 ]]; then
    echo "DELETE OK"
else
    echo "DELETE not OK"
    kill "$pid"
    exit 1
fi

get=$(curl http://localhost:5000/this/is/test)

if [[ $get =~ "0" ]]; then
    echo "GET OK"
else
    echo "GET not OK"
    kill "$pid"
    exit 1
fi

kill "$pid"