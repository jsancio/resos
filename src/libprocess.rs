use chan;
use hyper::server;
use hyper::client;
use hyper::header::{Connection, ContentType, Headers};
use hyper::method::Method::Post;
use hyper::status::StatusCode;
use hyper::uri::RequestUri::AbsolutePath;
use protobuf;
use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};
use std::io::Read;
use std::net::SocketAddr;
use std::str::FromStr;
use std::thread;

header! {
    (LibprocessFrom, "Libprocess-From") => [UPID]
}

lazy_static! {
    static ref ContentTypeProtobuf: ContentType = ContentType("application/x-protobuf".parse().unwrap());
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
        format!("http://{}/{}/", self.address, self.id)
    }
}

impl Display for UPID {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}@{}", self.id, self.address)
    }
}

impl FromStr for UPID {
    type Err = UPIDError;
    fn from_str(pid: &str) -> Result<UPID, Self::Err> {
        let mut split = pid.split("@");
        match (split.nth(0), split.nth(0)) {
            (Some(id), Some(address)) => {
                match FromStr::from_str(address) {
                    Ok(address) => Ok(UPID::new(id, address)),
                    Err(_) => Err(UPIDError)
                }
            },
            _ => Err(UPIDError)
        }
    }
}

#[derive(Debug)]
pub struct UPIDError;

pub struct LibProcess {
    http_server: server::Listening,
    http_client: client::Client,
    pid: UPID,
    rx: chan::Receiver<(String, UPID, Vec<u8>)>
}

impl LibProcess {
    pub fn new(id: &str) -> LibProcess {
        let (tx, rx) = chan::async();
        let http_server = LibProcess::new_server(id, tx);
        let http_client = client::Client::new();
        let pid = UPID::new(id, http_server.socket.clone());

        LibProcess{http_server: http_server,
                   http_client: http_client,
                   pid: pid,
                   rx: rx}
    }

    fn new_server(myid: &str, tx: chan::Sender<(String, UPID, Vec<u8>)>) -> server::Listening {
        let myid_len = myid.len() + 2;

        server::Server::http("0.0.0.0:0").unwrap()
                 .handle(move |req: server::Request,
                          mut resp: server::Response| {

            match req.deconstruct() {
                (_, Post, headers, AbsolutePath(path), _, mut body) => {
                    let id = &path[..myid_len];
                    let mut data = Vec::new();
                    if let Err(e) = body.read_to_end(&mut data) {
                        warn!("Failed to read message for {}: {}", id, e);
                        *resp.status_mut() = StatusCode::BadRequest;
                        return
                    }

                    let name = path[myid_len..].to_string();
                    let sender = match headers.get() {
                        Some(&LibprocessFrom(ref upid)) => upid.clone(),
                        None => panic!("No LibprocessFrom in header")
                    };

                    tx.send((name, sender, data));

                    *resp.status_mut() = StatusCode::Accepted;
                },
                (_, _, _, uri, _, _) => {
                    warn!("Unhandled {:?}", uri);
                }
            }
        }).unwrap()
    }

    pub fn send(&mut self,
                pid: &UPID,
                message: &protobuf::Message) -> Result<(), ()> {

        let mut url = pid.to_url();
        url.push_str(message.descriptor().full_name());

        let mut headers = Headers::new();
        headers.set(Connection::keep_alive());
        headers.set(ContentTypeProtobuf.clone());
        headers.set(LibprocessFrom(self.pid.clone()));

        let data = message.write_to_bytes().unwrap();

        let resp = self.http_client.post(&url)
                                   .headers(headers)
                                   .body(&data[..])
                                   .send();

        match resp {
            Ok(client::Response{status: StatusCode::Accepted, ..}) => Ok(()),
            _ => Err(())
        }
    }

    pub fn start<T: Send + 'static, F1, M>(&self, context: T, handlers: HashMap<String, F1>)
    where F1: Fn(UPID, M, &T) + Send + 'static,
           M: protobuf::Message + protobuf::MessageStatic {

        let rx = self.rx.clone();
        thread::spawn(move || {
            loop {
                match rx.recv() {
                    Some((name, sender, data)) => match protobuf::parse_from_bytes::<M>(&data) {
                        Ok(msg) => match handlers.get(&name) {
                            Some(handler) => handler(sender, msg, &context),
                            None => warn!("Unahandled message: {}", name)
                        },
                        Err(_) => error!("Failed to parse protobuf message from master")
                    },
                    None => () // context.join.wait(), handle error
                };
            }
        });
    }

    pub fn close(&mut self) {
        self.http_server.close().unwrap();
    }
}

trait LibProcessHandler<T>: Send {
    fn handle(sender: &UPID, data: &Vec<u8>, context: &T);
}
