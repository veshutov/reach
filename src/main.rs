mod model;
mod security;
mod web;

#[tokio::main]
async fn main() {
    let db = model::init_db()
        .await
        .unwrap();
    web::start_web(db)
        .await
        .unwrap();
}
