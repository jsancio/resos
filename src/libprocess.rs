use hyper;
use hyper::client::Response;
use hyper::header::{ContentType, Headers};
use hyper::status::StatusCode;
use hyper::uri::RequestUri::AbsolutePath;
use protobuf;
use protobuf::error::ProtobufError;
use std::fmt::{Display, Error, Formatter};
use std::io::Read;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::mpsc;
use std::sync::Mutex;

header! {
    (LibprocessFrom, "Libprocess-From") => [UPID]
}

#[derive(Clone, Debug, PartialEq)]
pub struct UPID {
    id: String,
    address: SocketAddr
}

impl UPID {
    fn new(id: &str, address: SocketAddr) -> UPID {
        UPID{id: id.to_string(), address: address}
    }

    fn to_url(&self) -> String {
        format!("http://{}/{}", self.address, self.id)
    }
}

impl Display for UPID {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}@{}", self.id, self.address)
    }
}

impl FromStr for UPID {
    type Err = UPIDError;
    fn from_str(s: &str) -> Result<UPID, Self::Err> {
        let mut split = s.split("@");
        match (split.nth(0), split.nth(0)) {
            (Some(id), Some(address)) => {
                match FromStr::from_str(address) {
                    Ok(address) => Ok(UPID::new(id, address)),
                    Err(e) => Err(UPIDError)
                }
            },
            _ => Err(UPIDError)
        }
    }
}

#[derive(Debug)]
pub struct UPIDError;

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
    pid: UPID
}

impl LibProcess {
    pub fn new(id: &str) -> (LibProcess, mpsc::Receiver<(String, Vec<u8>)>) {
        let (tx, rx) = mpsc::channel();
        let http_server = LibProcess::new_server(id, tx);
        let http_client = hyper::Client::new();
        let pid = UPID::new(id, http_server.socket.clone());

        (LibProcess{http_server: http_server,
                    http_client: http_client,
                    pid: pid},
                    rx)
    }

    fn new_server(myid: &str,
                  tx: mpsc::Sender<(String, Vec<u8>)>) -> hyper::server::Listening {
        let myid_len = myid.len() + 2;

        // Mutex needed because of Sync contstraint on Handler
        let gtx = Mutex::new(tx);

        hyper::Server::http(move |req: hyper::server::Request,
                                  mut resp: hyper::server::Response| {
            let (_, _, _, uri, _, mut body) = req.deconstruct();
            // TODO
            // - match for POST
            // - put sender into the message
            match uri {
                AbsolutePath(path) => {
                    let mut data = Vec::new();
                    body.read_to_end(&mut data).unwrap();
                    let res = gtx.lock().unwrap().send((path[myid_len..].to_string(), data));
                    debug!("HTTP Server got {:?}", res);
                    *resp.status_mut() = StatusCode::Accepted;
                },
                _ => {
                    warn!("Unhandled {:?}", uri);
                }
            }
        }).listen("0.0.0.0:0").unwrap()
    }

    pub fn send(&mut self,
                pid: &UPID,
                message: &protobuf::Message) -> Result<(), ()> {
        let mut url = pid.to_url();
        url.push_str("/mesos.internal.");
        url.push_str(message.descriptor().name());
        let mut headers = Headers::new();
        headers.set(ContentType("application/x-protobuf".parse().unwrap()));
        headers.set(LibprocessFrom(self.pid.clone()));
        let data = serialize(message).unwrap();

        let resp = self.http_client.post(&url)
              .headers(headers)
              .body(&data[..])
              .send();
        match resp {
            Ok(resp) => match resp.status {
                StatusCode::Accepted => Ok(()),
                status => Err(())
            },
            Err(err) => Err(())
        }
    }

    pub fn close(&mut self) {
        self.http_server.close().unwrap();
    }
}

// TODO I think it's not needed here
pub trait Handler : Send + Sync {
    fn handle(&self, name: &str, data: &Vec<u8>);
}
