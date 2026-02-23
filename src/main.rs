mod router;
mod handlers;

use std::{env, net::SocketAddr};

#[tokio::main]
async fn main(){


    let app = router::create_router();
      let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    // Bind to 0.0.0.0
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

