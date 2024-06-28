use axum::{extract::Query, response::Html, routing::get, Router};
use rand::{thread_rng, Rng};
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(get_home));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// `Deserialize` need be implemented to use with `Query` extractor.
#[derive(Deserialize)]
struct HomeParameters {
    firstname: String,
    lastname: String
}


async fn get_home(Query(pars): Query<HomeParameters>) -> Html<String> {
    let random_number = thread_rng().gen_range(0..10);

    // Send response in html format.
    Html(format!("<h1>Hey {} {}, your lucky number is {}!</h1>", pars.firstname, pars.lastname, random_number))
}
