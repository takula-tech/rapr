#![cfg_attr(not(unix), allow(unused_imports))]

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

use hello_world::{HelloRequest, greeter_client::GreeterClient};
use std::time::Instant;

#[cfg(unix)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Unix socket URI follows [RFC-3986](https://datatracker.ietf.org/doc/html/rfc3986)
    // which is aligned with [the gRPC naming convention](https://github.com/grpc/grpc/blob/master/doc/naming.md).
    // - unix:relative_path
    // - unix:///absolute_path
    let path = "unix:///tmp/uds_tcp_grpc";
    let mut client = GreeterClient::connect(path).await?;

    const REQUESTS: usize = 10_0000;
    let start = Instant::now();

    for i in 0..REQUESTS {
        let request = tonic::Request::new(HelloRequest {
            name: "a".repeat(1024),
        });
        let _response = client.say_hello(request).await?;
        // Optional: Print progress every 1000 requests
        if (i + 1) % 10_000 == 0 {
            println!("Completed {} requests", i + 1);
        }
    }

    let total_duration = start.elapsed();
    let avg_latency_ms = total_duration.as_secs_f64() * 1000.0 / REQUESTS as f64;
    println!("\n=== UDS gRPC Benchmark Results ===");
    println!("Total requests: {REQUESTS}");
    println!("Total time: {total_duration:?}");
    println!("Average latency: {avg_latency_ms:.3} ms");
    println!(
        "Requests per second: {:.2}",
        REQUESTS as f64 / total_duration.as_secs_f64()
    );

    // === UDS gRPC Benchmark Results ===
    // Total requests: 100000
    // Total time: 3.771999458s
    // Average latency: 0.038 ms
    // Requests per second: 26511.14

    Ok(())
}

#[cfg(not(unix))]
fn main() {
    panic!("The `uds` example only works on unix systems!");
}
