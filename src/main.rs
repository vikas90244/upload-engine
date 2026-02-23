mod router;
mod handlers;


#[tokio::main]
async fn main(){

    tokio::fs::create_dir_all("../../uploads").await.unwrap();

//     let app=Router::new().route("/", get(|| async {"Hello, world"} ));
    let app = router::create_router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
