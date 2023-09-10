use crate::db::Repository;
use crate::responses::error::ErrorResponse;
use crate::utils::auto_complete::get_suggestions;
use poem_openapi::{payload::Json, ApiResponse};

#[derive(ApiResponse)]
pub enum SearchDomainResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<String>>),

    #[oai(status = 500)]
    InternalServerError(Json<ErrorResponse>),
}

pub async fn search(_pool: &Repository, query: String) -> SearchDomainResponse {
    let mut auto_complete = get_suggestions(&query, 19);

    if !auto_complete.contains(&query) {
        auto_complete.insert(0, query);
    }

    SearchDomainResponse::Ok(Json(auto_complete))
}
