use http_body_util::BodyExt;
use http_body_util::Empty;
use hyper::body::Bytes;
use hyper::Request;
use hyper::StatusCode;
use hyper::{header::HeaderValue, HeaderMap};
use hyper_util::rt::TokioIo;
use thiserror::Error;
use tokio::io::{AsyncWriteExt as _, BufWriter};
use tokio::net::TcpStream;

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Http Error {0}")]
    HttpError(#[from] hyper::http::Error),

    #[error("Hyper Error {0}")]
    HyperError(#[from] hyper::Error),

    #[error("Io Error {0}")]
    IoError(#[from] std::io::Error),

    #[error("Utf8 Error {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("Uri Has No Host")]
    UriHasNoHost,
}

#[derive(Debug)]
pub struct Res {
    pub body: String,
    pub status: StatusCode,
    pub headers: HeaderMap<HeaderValue>,
}

pub async fn get(url: String, headers: &HeaderMap<HeaderValue>) -> Result<Res, RequestError> {
    let url = url.parse::<hyper::Uri>().unwrap();

    let host = match url.host() {
        Some(s) => s,
        None => return Err(RequestError::UriHasNoHost),
    };
    let port = url.port_u16().unwrap_or(80);

    let address = format!("{}:{}", host, port);
    let stream = TcpStream::connect(address).await?;
    let io = TokioIo::new(stream);
    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let authority = url.authority().unwrap().clone();

    let mut req = Request::builder()
        .uri(url)
        .header(hyper::header::HOST, authority.as_str())
        .body(Empty::<Bytes>::new())?;
    let req_headers = req.headers_mut();

    for header in headers.iter() {
        req_headers.insert(header.0, header.1.to_owned());
    }

    let mut res = sender.send_request(req).await?;
    let mut body = BufWriter::new(Vec::new());

    while let Some(next) = res.frame().await {
        let frame = next?;
        if let Some(chunk) = frame.data_ref() {
            body.write_all(chunk).await?;
        }
    }

    Ok(Res {
        body: String::from_utf8(body.buffer().to_vec())?,
        status: res.status(),
        headers: res.headers().clone(),
    })
}
