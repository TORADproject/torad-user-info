use std::{io, sync::Arc};

use actix_web::{
    middleware, route,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use juniper::http::GraphQLRequest;
mod environment;
mod schema;
use crate::{
    environment::validate_environment,
    schema::{create_schema, Schema},
};
use dotenv::dotenv;

const BIND_ADDR: &str = "0.0.0.0";
const BIND_PORT: u16 = 8080;

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(schema: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let user = data.execute(&schema, &()).await;
    HttpResponse::Ok().json(user)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    validate_environment().unwrap();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Create Juniper schema
    let schema = Arc::new(create_schema());

    log::info!("GraphQL endpoint: http://{BIND_ADDR}:{BIND_PORT}/graphql");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .service(graphql)
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind((BIND_ADDR, BIND_PORT))?
    .run()
    .await
}
