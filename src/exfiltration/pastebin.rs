extern crate hyper;
extern crate regex;

use hyper::client::Request;
use hyper::Url;
use regex::Regex;

fn verify_data(key: str) -> bool {
    let re = Regex::new(r"[0-9a-f]{32}").unwrap();
    return re.is_match(key);
}
fn login(key: str, unam: str, pass: str) -> str {
    let url = match Url::parse("https://pastebin.com/api/api_login.php?=api_dev_key={0}&api_user_name={1}&api_user_password={2}",
        key,
        unam,
        pass
    ) {
        Ok(url) => url,
        Err(_) => panic!("Invalid URL"),
    };
    let fresh_request = match Request::get(url) {
        Ok(request) => request,
        Err(_) => panic!("Get failed."),
    };
    let streaming_request = match fresh_request.start() {
        Ok(request) => request,
        Err(_) => panic!("Stream request failed."),
    };
    let mut response = match streaming_request.send() {
        Ok(response) => response,
        Err(_) => panic!("Response aquisition failed."),
    };
    let apikey = match response.read_to_string() {
        Ok(apikey) => apikey,
        Err(_) => panic!("Request parsing failed / response is NULL."),
    };
    return apikey
}
fn paste(key: str, val: str) -> bool {
    let url = match Url::parse("http://pastebin.com/api/api_post.php?=api_option=paste&api_dev_key={0}&api_paste_code={1}",
        key,
        val
    ) {
        Ok(url) => url,
        Err(_) => panic!("Invalid URL"),
    };
    let fresh_request = match Request::get(url) {
        Ok(request) => request,
        Err(_) => panic!("Get failed."),
    };
    let streaming_request = match fresh_request.start() {
        Ok(request) => request,
        Err(_) => panic!("Stream request failed."),
    };
    let mut response = match streaming_request.send() {
        Ok(response) => response,
        Err(_) => panic!("Response aquisition failed."),
    };
    let content = match response.read_to_string() {
        Ok(content) => content,
        Err(_) => panic!("Request parsing failed / response is NULL."),
    };
    println!("{}", content);
    return match Url::parse(content) {
        Ok(resurl) => resurl = true,
        Err(_) => resurl = false,
    };
}
