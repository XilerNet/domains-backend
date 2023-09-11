#![feature(async_fn_in_trait)]

use db::DomainRepository;
use endpoints::search::SearchDomainResponse;
use poem::{
    listener::TcpListener, middleware::Cors, web::Data, EndpointExt, Result, Route, Server,
};
use poem_openapi::param::Query;
use poem_openapi::{OpenApi, OpenApiService};
use std::env;

use crate::db::Repository;
use crate::utils::auto_complete;

pub mod db;
pub mod endpoints;
pub mod responses;
pub mod utils;

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/search", method = "get")]
    async fn search(&self, pool: Data<&Repository>, search: Query<String>) -> SearchDomainResponse {
        endpoints::search::search(&pool, search.0).await
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install().ok();
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    auto_complete::initialize();

    let repository = Repository::new().await;

    let api_service = OpenApiService::new(Api, "Xiler Domains API", "v0.0.1")
        .server("http://localhost:25201")
        .server("https://domains-api.xiler.net");
    let open_api = api_service.swagger_ui();

    let origins: Vec<String> = if cfg!(debug_assertions) {
        vec![
            env::var("DEV_MAIN_URL").expect("DEV_MAIN_URL not set"),
            "http://localhost:25201".to_string(),
        ]
    } else {
        vec![env::var("PROD_MAIN_URL").expect("PROD_MAIN_URL not set")]
    };

    let routes = Route::new()
        .nest("/", api_service)
        .nest("/swagger", open_api)
        .with(Cors::new().allow_origins(origins))
        .data(repository);

    Server::new(TcpListener::bind("127.0.0.1:25201"))
        .run(routes)
        .await?;

    Ok(())
}
