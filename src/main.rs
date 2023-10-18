mod server;
use server::{CustomSocketAddress, run_server};

#[tokio::main]
async fn main() {
    let addr = CustomSocketAddress {
        octets : vec![127, 0, 0, 1],
        port   : 8080,
        };

    run_server(addr).await; 
}