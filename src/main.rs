#![feature(async_fn_in_trait)]

use db::DomainRepository;
use endpoints::about::AboutInscriptionResponse;
use endpoints::domains::{DomainsData, DomainsResponse};
use endpoints::search::SearchDomainResponse;
use poem::web::Data;
use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Result, Route, Server};
use poem_openapi::param::{Path, Query};
use poem_openapi::payload::Json;
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
    async fn search(&self, pool: Data<&Repository>, query: Query<String>) -> SearchDomainResponse {
        endpoints::search::search(&pool, query.0).await
    }

    #[oai(path = "/domains", method = "post")]
    async fn domains(&self, pool: Data<&Repository>, data: Json<DomainsData>) -> DomainsResponse {
        endpoints::domains::domains(&pool, &data).await
    }

    #[oai(path = "/about/:id", method = "get")]
    async fn about(&self, pool: Data<&Repository>, id: Path<String>) -> AboutInscriptionResponse {
        endpoints::about::about(&pool, id.0).await
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
