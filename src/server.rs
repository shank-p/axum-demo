use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use axum::{Router, routing::get, response::Html};

pub struct CustomSocketAddress {
    pub octets : Vec<u8>,
    pub port   : u16,
}
impl CustomSocketAddress {
    fn to_ip_addr(&self) -> Result<Ipv4Addr, &'static str> {
        match self.octets.len() {
            4 => {
                if let [a, b, c, d] = &self.octets[..] {
                    let ipv4_addr = Ipv4Addr::new(*a, *b, *c, *d);
                    return Ok(ipv4_addr);
                } else {
                    return Err("Unsupported address");
                }
            },
            _ => {
                return Err("Unsupported address");
            }
        }
    }
}

pub async fn run_server(addr: CustomSocketAddress) {

    let app = Router::new()
        .route("/", get(|| async {Html("Hello World")}));

    let ip_addr = match addr.to_ip_addr() {
        Ok(addr) => IpAddr::V4(addr),
        Err(error) => {
            println!("{error}");
            std::process::exit(1);
        }
    };
    let socket_addr = SocketAddr::new(ip_addr, addr.port);

    println!(" ----- SERVER INITIALIZED -----");
    println!(" > Listening on address : {socket_addr}");
    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}