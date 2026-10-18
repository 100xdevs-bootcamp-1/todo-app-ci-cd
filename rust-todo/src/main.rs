#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".into());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    println!("rust-todo listening on :{port}");
    axum::serve(listener, rust_todo::app()).await.unwrap();
}
