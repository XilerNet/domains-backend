use poem_openapi::{payload::Json, ApiResponse, Object};

use crate::db::{DomainRepository, Repository};
use crate::responses::error::ErrorResponse;

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct DomainsData {
    addresses: Vec<String>,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct DomainsResponseDomain {
    domain: String,
    inscription: String,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct DomainsResponseObject {
    domains: Vec<DomainsResponseDomain>,
}

#[derive(ApiResponse)]
pub enum DomainsResponse {
    #[oai(status = 200)]
    Ok(Json<DomainsResponseObject>),

    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),

    #[oai(status = 500)]
    InternalServerError(Json<ErrorResponse>),
}

pub async fn domains(pool: &Repository, data: &DomainsData) -> DomainsResponse {
    let domains = pool
        .get_domains_of_addresses(&data.addresses)
        .await
        .into_iter()
        .map(|(domain, inscription)| DomainsResponseDomain {
            domain,
            inscription,
        })
        .collect();

    DomainsResponse::Ok(Json(DomainsResponseObject { domains }))
}
