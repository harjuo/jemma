use decipher::get_operation;

fn main() {
    let parcel = "GET /foo/bar/baz/ HTTP/1.1";
    let op = get_operation(parcel);
    println!("{:?}", op);
}
