#![cfg_attr(not(unix), allow(unused_imports))]

#[cfg(unix)]
use std::fs;
use std::path::Path;
#[cfg(unix)]
use tokio::net::UnixListener;
#[cfg(unix)]
use tokio::signal;
#[cfg(unix)]
use tokio_stream::wrappers::UnixListenerStream;
use tonic::{Request, Response, Status, transport::Server};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

use hello_world::{
    HelloReply, HelloRequest,
    greeter_server::{Greeter, GreeterServer},
};

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[cfg(unix)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "/tmp/uds_tcp_grpc";

    // Create directory if it doesn't exist
    std::fs::create_dir_all(Path::new(path).parent().unwrap())?;

    // Remove existing socket file if it exists
    if Path::new(path).exists() {
        fs::remove_file(path)?;
        println!("Removed existing socket file: {path}");
    }

    let greeter = MyGreeter::default();

    let uds = UnixListener::bind(path)?;
    let uds_stream = UnixListenerStream::new(uds);

    println!("gRPC server listening on Unix socket: {path}");

    // Set up signal handling for graceful shutdown
    let server = Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve_with_incoming(uds_stream);

    // Handle shutdown signals
    tokio::select! {
        result = server => {
            if let Err(err) = result {
                eprintln!("Server error: {err}");
            }
        }
        _ = signal::ctrl_c() => {
            println!("\nReceived Ctrl+C, shutting down gracefully...");
        }
        _ = terminate_signal() => {
            println!("\nReceived terminate signal, shutting down gracefully...");
        }
    }

    // Clean up the socket file
    if Path::new(path).exists() {
        fs::remove_file(path)?;
        println!("Cleaned up socket file: {path}");
    }

    Ok(())
}

#[cfg(unix)]
async fn terminate_signal() -> Result<(), std::io::Error> {
    signal::unix::signal(signal::unix::SignalKind::terminate())?
        .recv()
        .await;
    Ok(())
}

#[cfg(not(unix))]
fn main() {
    panic!("The `uds` example only works on unix systems!");
}
