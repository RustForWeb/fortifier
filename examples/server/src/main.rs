mod email_address;
mod routes;
mod user;

use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use tokio::net::TcpListener;

use crate::routes::Routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let router = Routes::router();

    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let listener = TcpListener::bind(&address).await?;

    println!("listening on http://{}", &address);
    axum::serve(listener, router).await?;

    Ok(())
}
