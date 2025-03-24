use std::convert::Infallible;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Request, Response, header};


pub async fn response(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::builder()
        .status(200)
        .header(header::CONTENT_TYPE, "text/html")
        .body(Full::new(Bytes::from("<h1>Hello!<h1/>"))).unwrap())

}

