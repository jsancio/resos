use hyper::Client;
use hyper::header::Connection;
use proto::FrameworkInfo;
use protobuf::Message;
use utils;

pub fn request(url: &str, message: &FrameworkInfo) {
    let data = utils::serialize(message);
    let mut client = Client::new();
    let res = client.get(url)
        .send().unwrap();
}