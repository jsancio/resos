use hyper;
use hyper::client::Response;
use hyper::header::{ContentType, Headers};
use hyper::status::StatusCode;
use hyper::uri::RequestUri::AbsolutePath;
use protobuf;
use protobuf::error::ProtobufError;
use std::io::Read;
use std::net::SocketAddr;
use std::sync::mpsc;
use std::sync::Mutex;

header! {
    (LibprocessFrom, "Libprocess-From") => [String]
}

pub struct UPID {
    id: String,
    address: SocketAddr
}

impl ToString for UPID {
    fn to_string(&self) -> String {
        format!("{}@{}", self.id, self.address)
    }
}

impl UPID {
    fn new(id: &str, address: &SocketAddr) -> UPID {
        UPID{id: id.to_string(), address: address.clone()}
    }
}

/// Returns the result of serializing the supplied protobuf message
pub fn serialize(proto: &protobuf::Message) -> Result<Vec<u8>, ProtobufError> {
    proto.write_to_bytes()
}

/// Returns the result of deserializing the supplied protobuf data
/// into the supplied message.
// pub fn deserialize<'a, T: protobuf::Message>(obj: &Vec<u8>,
//                                              proto: &'a mut T) -> Result<&'a mut T, ProtobufError> {
//     try!(proto.merge_from_bytes(obj));
//     Ok(proto)
// }

pub struct LibProcess {
    http_server: hyper::server::Listening,
    http_client: hyper::Client,
    master: String,
    pid: UPID
}

impl LibProcess {
    pub fn new(master: &str, id: &str) -> (LibProcess, mpsc::Receiver<(String, Vec<u8>)>) {
        let (tx, rx) = mpsc::channel();
        let http_server = LibProcess::new_server(tx);
        let http_client = hyper::Client::new();
        let pid = UPID::new(id, &http_server.socket);
        let id_end = pid.id.len() + 2;

        //         // slice the id from the path
        //         //match &path[id_end..] {
        //             // "/mesos.internal.FrameworkRegisteredMessage" => {
        //             //    let message: FrameworkRegisteredMessage = parse_from_bytes(&data).unwrap();
        //             //     println!("FrameworkRegisteredMessage {:?}", message);
        //             //     scheduler.registered(&*driver_arc, message.get_framework_id(), message.get_master_info());
        //             // },
        //             // message => {
        //             //     println!("Unhandled {:?}", message);
        //             // }
        //         //}

        (LibProcess{http_server: http_server, http_client: http_client, master: master.to_string(), pid: pid}, rx)
    }

    fn new_server(tx: mpsc::Sender<(String, Vec<u8>)>) -> hyper::server::Listening {
        let gtx = Mutex::new(tx); // TODO lock needed because of Sync contstraint on Handler
        hyper::Server::http(move |req: hyper::server::Request,
                                  mut resp: hyper::server::Response| {
            let (_, _, _, uri, _, mut body) = req.deconstruct();
            match uri {
                AbsolutePath(path) => {
                    let mut data = Vec::new();
                    body.read_to_end(&mut data).unwrap();
                    let res = gtx.lock().unwrap().send((path, data));
                    println!("HTTP Server got {:?}", res);
                    *resp.status_mut() = StatusCode::Accepted;
                },
                _ => {
                    println!("Unhandled {:?}", uri);
                }
            }
        }).listen("0.0.0.0:0").unwrap()
    }

    pub fn send(&mut self,
                message: &protobuf::Message) -> hyper::error::Result<hyper::client::Response> {
        let mut uri = self.master.to_string();
        uri.push_str("/mesos.internal.");
        uri.push_str(message.descriptor().name());
        let mut headers = Headers::new();
        headers.set(ContentType("application/x-protobuf".parse().unwrap()));
        headers.set(LibprocessFrom(self.pid.to_string()));
        let data = serialize(message).unwrap();
        let res = self.http_client.post(uri.trim())
              .headers(headers)
              .body(&data[..])
              .send();
        res
    }

    pub fn close(&mut self) {
        self.http_server.close();
    }
}

pub trait Handler : Send + Sync {
    fn handle(&self, name: &str, data: &Vec<u8>);
}
