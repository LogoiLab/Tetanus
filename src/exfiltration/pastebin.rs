extern crate hyper;
extern crate regex;

use hyper::client::Request;
use hyper::Url;

fn verify_data(opt: str, key: str) {
    ([0-9a-f]{32})
}
fn create_paste_uri(opt: str, key: str, val: str, ) {
    let url = match Url::parse("http://pastebin.com/api/api_post.php?=api_option={0}&api_dev_key={1}&api_paste_code={2}", opt, key, val)
}

fn main() {
    let url = match Url::parse("http://httpbin.org/status/200") {
        Ok(url) => url,
        Err(_) => panic!("Uh oh."),
    };
    println!("> get: {}", url);
    let fresh_request = match Request::get(url) {
        Ok(request) => request,
        Err(_) => panic!("Whoops."),
    };
    let streaming_request = match fresh_request.start() {
        Ok(request) => request,
        Err(_) => panic!("Noooo."),
    };
    let mut response = match streaming_request.send() {
        Ok(response) => response,
        Err(_) => panic!("So close..."),
    };
    println!("< status code: {}", response.status);
    let content = match response.read_to_string() {
        Ok(content) => content,
        Err(_) => panic!("I give up."),
    };
    println!("{}", content);
}
