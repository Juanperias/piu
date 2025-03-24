mod config;
mod cli;

use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use cli::Cli;
use config::read_config;
use clap::Parser;
use tracing_subscriber::FmtSubscriber;
use tracing::{info, error, Level};

async fn response(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello from piu!"))))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Cli::parse();
    let config = read_config(args.config).await?;
    let result: [u8; 4] = config.host_config.host.split('.').map(|x| x.parse().unwrap()).collect::<Vec<_>>().try_into().unwrap();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;
    let addr = SocketAddr::from((result, config.host_config.port)); 

    info!("Starting piu at port {}", config.host_config.port);

    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(response))
                .await
            {
                error!("Error serving connection {:?}", err);
            }
        });
    }
}
