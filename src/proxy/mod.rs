use crate::config::BindConfig;
use crate::http::req::{get, RequestError};
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{header, Request, Response};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResponseError {
    #[error("Http Error {0}")]
    HttpError(#[from] hyper::http::Error),

    #[error("Request Error {0}")]
    RequestError(#[from] RequestError),
}

pub async fn response(
    req: Request<hyper::body::Incoming>,
    binds: HashMap<String, BindConfig>,
) -> Result<Response<Full<Bytes>>, ResponseError> {
    let uri: String = req.uri().to_string().chars().skip(1).collect();

    if let Some(bind) = binds.get(&uri) {
    let res = get(bind.proxy_pass.to_owned(), req.headers()).await?;
    Ok(Response::builder()
        .status(res.status.as_u16())
        .header(
            header::CONTENT_TYPE,
            res.headers.get("Content-Type").unwrap(),
        )
        .body(Full::new(Bytes::from(res.body)))?)
    } else {
      Ok(Response::builder()
         .status(404)
         .header(header::CONTENT_TYPE, "text/html")
         .body(Full::new(Bytes::from("<h1>404 url not found</h1>")))?
      )  
    }
}
