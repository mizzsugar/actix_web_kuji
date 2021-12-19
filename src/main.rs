use actix_web::{web, App, HttpResponse, HttpServer};
use rand::seq::SliceRandom;
use serde::Deserialize;

async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Deserialize)]
struct Kuji {
    names: Vec<String>,
}

async fn kuji(request_body: web::Json<Kuji>) -> HttpResponse {
    let mut rng = rand::thread_rng();
    HttpResponse::Ok().body(request_body.names.choose(&mut rng).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(hello))
            .route("/kuji", web::post().to(kuji))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use serde_json::json;

    #[actix_rt::test]
    async fn test_kuji() {
        let mut app = test::init_service(App::new().route("/kuji", web::post().to(kuji))).await;
        let request_body = json!({ "names": vec!["a", "b", "c"] });
        let choices = vec![
            web::Bytes::from("a"),
            web::Bytes::from("b"),
            web::Bytes::from("c"),
        ];
        let req = test::TestRequest::post()
            .uri("/kuji")
            .set_json(&request_body)
            .to_request();
        let resp = test::read_response(&mut app, req).await;
        assert!(choices.contains(&resp));
    }
}
