use hyper::server;
use hyper::client;
use hyper::header::{Connection, ContentType, Headers};
use hyper::status::StatusCode;
use hyper::uri::RequestUri::AbsolutePath;
use protobuf;
use std::fmt::{Display, Error, Formatter};
use std::io::Read;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::mpsc;
use std::sync::Mutex;

header! {
    (LibprocessFrom, "Libprocess-From") => [UPID]
}

static PROTOBUF: &'static str = "application/x-protobuf";

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
    pid: UPID
}

impl LibProcess {
    pub fn new(id: &str) -> (LibProcess, mpsc::Receiver<(String, Vec<u8>)>) {
        let (tx, rx) = mpsc::channel();
        let http_server = LibProcess::new_server(id, tx);
        let http_client = client::Client::new();
        let pid = UPID::new(id, http_server.socket.clone());

        (LibProcess{http_server: http_server,
                    http_client: http_client,
                    pid: pid},
                    rx)
    }

    fn new_server(myid: &str,
                  tx: mpsc::Sender<(String, Vec<u8>)>) -> server::Listening {
        let myid_len = myid.len() + 2;

        // Mutex needed because of Sync contstraint on Handler
        let sender = Mutex::new(tx);

        server::Server::http(move |req: server::Request,
                              mut resp: server::Response| {
            let (_, _, _, uri, _, mut body) = req.deconstruct();
            // TODO
            // - match for POST
            // - put sender into the message
            match uri {
                AbsolutePath(path) => {
                    let mut data = Vec::new();
                    let id = &path[..myid_len];
                    if let Err(e) = body.read_to_end(&mut data) {
                        warn!("Failed to read message for {}: {}", id, e);
                        *resp.status_mut() = StatusCode::BadRequest;
                        return
                    }
                    let sender = sender.lock().unwrap();
                    let name = path[myid_len..].to_string();
                    if let Err(e) = sender.send((name, data)) {
                        warn!("Failed to send message to {}: {}", id, e);
                    }
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
        url.push_str(message.descriptor().full_name());

        let mut headers = Headers::new();
        headers.set(Connection::keep_alive());
        headers.set(ContentType(PROTOBUF.parse().unwrap()));
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

    pub fn close(&mut self) {
        self.http_server.close().unwrap();
    }
}

// TODO I don't think it's needed here
pub trait Handler : Send + Sync {
    fn handle(&self, name: &str, data: &Vec<u8>);
}
