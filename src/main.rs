mod cli;
mod config;
mod http;
mod proxy;

use std::net::SocketAddr;
use std::sync::Arc;

use clap::Parser;
use cli::Cli;
use config::read_config;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use proxy::response;
use tokio::net::TcpListener;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Cli::parse();
    let config = Arc::new(read_config(args.config).await?);
    let result: [u8; 4] = config
        .host_config
        .host
        .split('.')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;
    let addr = SocketAddr::from((result, config.host_config.port));

    println!("{:?}", config);
    info!("Starting piu at port {}", config.host_config.port);

    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);
        let config = Arc::clone(&config);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(|req| response(req, config.bind.clone())))
                .await
            {
                error!("Error serving connection {:?}", err);
            }
        });
    }
}
