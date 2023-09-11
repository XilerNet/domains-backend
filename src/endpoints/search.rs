use crate::db::{DomainRepository, Repository};
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

pub async fn search(pool: &Repository, query: String) -> SearchDomainResponse {
    let query = query.trim_end_matches(".o").to_string();

    let mut auto_complete = get_suggestions(&query, 19);

    if !auto_complete.contains(&query) {
        auto_complete.insert(0, query);
    }

    // Add a `.o` suffix to each domain name
    auto_complete = auto_complete
        .into_iter()
        .map(|name| format!("{}.o", name))
        .collect();

    pool.retain_available_domain_names(&mut auto_complete).await;

    SearchDomainResponse::Ok(Json(auto_complete))
}
