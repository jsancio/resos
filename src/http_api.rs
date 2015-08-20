use hyper;
use hyper::header::{Accept, ContentType, Headers, qitem};
use hyper::status::StatusCode;
use protobuf;
use protobuf::error::ProtobufError;
use std::io::{Read, BufRead, BufReader};
use std::thread::{JoinHandle, spawn};

const PROTOBUF: &'static str = "application/x-protobuf";

lazy_static! {
    static ref AcceptProtobuf: Accept = Accept(vec![qitem(PROTOBUF.parse().unwrap())]);
    static ref ContentTypeProtobuf: ContentType = ContentType(PROTOBUF.parse().unwrap());
}

#[derive(Debug)]
pub enum Error {
    Io(hyper::Error),
    Status(StatusCode, String),
    Serialization(ProtobufError),
    Join
}

pub type Result<T> = ::std::result::Result<T, Error>;

pub struct HttpApi {
    http_client: hyper::Client,
    endpoint: String,
    join_handle: Option<JoinHandle<()>>
}

pub trait HttpHandler<Event: protobuf::MessageStatic> {
    fn on_event(&self, event: Event);
    fn on_error(&self, error: Error);
}

impl HttpApi {
    pub fn new(endpoint: &str) -> Result<Self> {
        let http_client = hyper::Client::new();

        Ok(HttpApi{http_client: http_client, endpoint: endpoint.to_string(), join_handle: None})
    }

    pub fn send(&mut self,
                to: &String,
                message: &protobuf::Message) -> Result<()> {
        match self.send_internal(to, message) {
            Ok(hyper::client::Response{status: StatusCode::Accepted, ..}) => Ok(()),
            Ok(resp) => Err(Error::Status(resp.status, resp.chars().map(|c| c.unwrap()).collect())),
            Err(e) => Err(e)
        }
    }

    pub fn join(&mut self) -> Result<()> {
        match self.join_handle.take() {
            Some(join_handle) => Ok(try!(join_handle.join().map_err(|_|Error::Join))),
            None => Ok(())
        }
    }

    fn send_internal(&mut self,
                     to: &String,
                     message: &protobuf::Message) -> Result<hyper::client::Response> {

        let mut url = "http://".to_string();
        url.push_str(to);
        url.push_str(&self.endpoint);

        let mut headers = Headers::new();
        headers.set(AcceptProtobuf.clone());
        headers.set(ContentTypeProtobuf.clone());

        let data = try!(message.write_to_bytes().map_err(Error::Serialization));

        self.http_client.post(&url)
                        .headers(headers)
                        .body(&data[..])
                        .send()
                        .map_err(Error::Io)
    }

    pub fn subscribe<Subscribe, Event, Handler>(&mut self,
                                                to: &String,
                                                subscribe: Subscribe,
                                                handler: Handler)-> Result<()>
    where Subscribe: protobuf::MessageStatic,
          Event: protobuf::MessageStatic,
          Handler: HttpHandler<Event> + Send + 'static {

        let resp = try!(self.send_internal(to, &subscribe));
        if resp.status == StatusCode::Ok {
            self.join_handle = Some(spawn(move || {
                let mut stream = BufReader::new(resp);
                loop {
                    match stream.read_message::<Event>() {
                        Ok(event) => handler.on_event(event),
                        Err(err) => handler.on_error(Error::Serialization(err))
                    }
                }
            }));
            Ok(())
        } else {
            Err(Error::Status(resp.status, resp.chars().map(|c| c.unwrap()).collect()))
        }
    }
}

trait ChunkedEncodedProtobufReader {
    fn read_message<M: protobuf::Message + protobuf::MessageStatic>(&mut self) -> protobuf::ProtobufResult<M>;
}

impl <R: BufRead> ChunkedEncodedProtobufReader for R {
    fn read_message<M: protobuf::Message + protobuf::MessageStatic>(&mut self) -> protobuf::ProtobufResult<M> {
        let mut line = String::new();
        let len_len = try!(self.read_line(&mut line).map_err(ProtobufError::IoError));
        let len: usize = try!(line[..len_len-1].parse().map_err(|_|ProtobufError::WireError("".to_string())));
        let mut buf = vec![0; len];
        let read_len = try!(self.read(&mut buf).map_err(ProtobufError::IoError));
        debug!("len = {:?} read_len = {:?}", len, read_len);
        protobuf::parse_from_bytes(&buf)
    }
}
