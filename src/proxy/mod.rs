use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Request, Response, header};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResponseError {
    #[error("Http Error {0}")]
    HttpError(#[from] hyper::http::Error)
}

pub async fn response(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, ResponseError> {
    Ok(Response::builder()
        .status(200)
        .header(header::CONTENT_TYPE, "text/html")
        .body(Full::new(Bytes::from("<h1>Hello from piu!<h1/>")))?)

}

