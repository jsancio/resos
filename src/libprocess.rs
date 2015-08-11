use chan;
use hyper;
use hyper::server;
use hyper::client;
use hyper::header::{Connection, ContentType, Headers};
use hyper::method::Method::Post;
use hyper::status::StatusCode;
use hyper::uri::RequestUri::AbsolutePath;
use protobuf;
use std::collections::HashMap;
use std::fmt;
use std::io::Read;
use std::net::SocketAddr;
use std::str::FromStr;
use std::thread::{JoinHandle, spawn};

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
    fn new(id: &str, address: SocketAddr) -> Self {
        UPID{id: id.to_string(), address: address}
    }

    fn to_url(&self) -> String {
        format!("http://{}/{}/", self.address, self.id)
    }
}

impl fmt::Display for UPID {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        write!(fmt, "{}@{}", self.id, self.address)
    }
}

impl FromStr for UPID {
    type Err = Error;
    fn from_str(pid: &str) -> Result<UPID> {
        let mut split = pid.split("@");
        match (split.nth(0), split.nth(0)) {
            (Some(id), Some(address)) => {
                match FromStr::from_str(address) {
                    Ok(address) => Ok(UPID::new(id, address)),
                    Err(_) => Err(Error::Upid)
                }
            },
            _ => Err(Error::Upid)
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Http(hyper::error::Error),
    Upid,
    Serialization
}

pub type Result<T> = ::std::result::Result<T, Error>;

type Handler<Context> = Box<Fn(UPID, Vec<u8>, &Context) + Send>;

pub struct HandlerMap<Context> {
    map: HashMap<String, Handler<Context>>,
    context: Context
}

impl<Context> HandlerMap<Context> {
    pub fn new(context: Context) -> Self {
        HandlerMap{map: HashMap::new(), context: context}
    }

    pub fn on<F, M>(&mut self, name: &str, handler: F)
    where F: Fn(UPID, M, &Context) + Send + 'static,
          M: protobuf::MessageStatic {

        self.map.insert(name.to_string(), Self::wrap(handler));
    }

    fn wrap<F, M>(handler: F) -> Handler<Context>
    where F: Fn(UPID, M, &Context) + Send + 'static,
          M: protobuf::MessageStatic {

        Box::new(move |sender: UPID, data: Vec<u8>, context: &Context| {
            match protobuf::parse_from_bytes::<M>(&data) {
                Ok(message) => {
                    debug!("Received {} {:?}", message.descriptor().name(), message);
                    handler(sender, message, context);
                },
                _ => error!("Failed to parse protobuf message from master")
            }
        })
    }
}

pub struct LibProcess {
    http_server: server::Listening,
    http_client: client::Client,
    pid: UPID,
    rx: chan::Receiver<(String, UPID, Vec<u8>)>
}

impl LibProcess {
    pub fn new(id: &str) -> Result<Self> {
        let (tx, rx) = chan::async();
        let http_server = try!(Self::new_server(id, tx));
        let http_client = client::Client::new();
        let pid = UPID::new(id, http_server.socket.clone());

        Ok(LibProcess{http_server: http_server,
                      http_client: http_client,
                      pid: pid,
                      rx: rx})
    }

    fn new_server(myid: &str, tx: chan::Sender<(String, UPID, Vec<u8>)>) -> Result<server::Listening> {
        let myid_len = myid.len() + 2;

        let server = try!(server::Server::http("0.0.0.0:0").map_err(Error::Http));

        server.handle(move |req: server::Request, mut resp: server::Response| {
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
        }).map_err(Error::Http)
    }

    pub fn send(&mut self,
                pid: &UPID,
                message: &protobuf::Message) -> Result<()> {

        let mut url = pid.to_url();
        url.push_str(message.descriptor().full_name());

        let mut headers = Headers::new();
        headers.set(Connection::keep_alive());
        headers.set(ContentTypeProtobuf.clone());
        headers.set(LibprocessFrom(self.pid.clone()));

        let data = try!(message.write_to_bytes().map_err(|_| Error::Serialization));

        let resp = self.http_client.post(&url)
                                   .headers(headers)
                                   .body(&data[..])
                                   .send();

        match resp {
            Ok(client::Response{status: StatusCode::Accepted, ..}) => Ok(()),
            Ok(_) => Err(Error::Http(hyper::error::Error::Status)),
            Err(e) => Err(Error::Http(e))
        }
    }

    pub fn start<Context>(&self, handlers: HandlerMap<Context>) -> JoinHandle<()>
    where Context: Send + 'static {

        let rx = self.rx.clone();
        spawn(move || {
            loop {
                match rx.recv() {
                    Some((name, sender, data)) => match handlers.map.get(&name) {
                        Some(handler) => handler(sender, data, &handlers.context),
                        None => warn!("Unahandled message: {}", name)
                    },
                    None => return
                }
            }
        })
    }

    pub fn close(&mut self) -> Result<()> {
        self.http_server.close().map_err(Error::Http)
    }
}
