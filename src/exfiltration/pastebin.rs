extern crate http;

fn main() {
    let request = RequestWriter::new(Get, FromStr::from_str(os::args()[1]).unwrap());
    let response = match request.read_response() {
        Ok(response) => response,
        Err(_request) => unreachable!(), // Uncaught condition will have failed first
    };
    if response.status == status::Ok {
        println!("Oh goodie, I got me a 200 OK response!");
    } else {
        println!("That URL ain't returning 200 OK, it returned {} instead", response.status);
    }
}
