use crate::db::{DomainRepository, Repository};
use crate::responses::error::ErrorResponse;
use poem_openapi::Object;
use poem_openapi::{payload::Json, ApiResponse};
use tracing::error;

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct AboutInscription {
    domain: String,
    inscription: String,
}

#[derive(ApiResponse)]
pub enum AboutInscriptionResponse {
    #[oai(status = 200)]
    Ok(Json<AboutInscription>),

    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),

    #[oai(status = 500)]
    InternalServerError(Json<ErrorResponse>),
}

pub async fn about(pool: &Repository, inscription: String) -> AboutInscriptionResponse {
    let inscription_query = inscription.trim_end_matches("i0");
    let res = pool.get_domain_by_inscription(&inscription_query).await;

    match res {
        Ok(Some(domain)) => {
            let domain = AboutInscription {
                domain,
                inscription,
            };

            AboutInscriptionResponse::Ok(Json(domain))
        }
        Ok(None) => AboutInscriptionResponse::NotFound(Json("Domain not found!".into())),
        Err(err) => {
            error!("[DB] Error getting domain by inscription: {:?}", err);
            AboutInscriptionResponse::InternalServerError(Json("Internal server error".into()))
        }
    }
}
