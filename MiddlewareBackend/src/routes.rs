use crate::types::{AppState, Nft};
use actix_web::{get, web, HttpResponse, Responder};
use diesel::prelude::*;


#[get("/ping")]
pub async fn ping_pong() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[get("/query1")]
pub async fn query1(app_data: web::Data<AppState>) -> impl Responder {
    use crate::schema::nft::dsl::*;

    let db_connection = app_data.pool.get().unwrap();

    let results = nft
        .limit(5)
        .load::<Nft>(&db_connection)
        .expect("Error loading nft");

    HttpResponse::Ok().json(results)

}
