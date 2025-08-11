use hello_world::HelloRequest;
use hello_world::greeter_client::GreeterClient;
use std::time::Instant;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

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
    println!("\n=== TCP gRPC Benchmark Results ===");
    println!("Total requests: {REQUESTS}");
    println!("Total time: {total_duration:?}");
    println!("Average latency: {avg_latency_ms:.3} ms");
    println!(
        "Requests per second: {:.2}",
        REQUESTS as f64 / total_duration.as_secs_f64()
    );

    // === TCP gRPC Benchmark Results ===
    // Total requests: 100000
    // Total time: 8.761265459s
    // Average latency: 0.088 ms
    // Requests per second: 11413.88

    Ok(())
}
