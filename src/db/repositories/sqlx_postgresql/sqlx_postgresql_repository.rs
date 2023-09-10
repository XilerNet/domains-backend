use sqlx::PgPool;
use tracing::debug;

use crate::db::DomainRepository;

// TODO: Handle multiple databases
#[derive(Clone)]
pub struct SqlxPostgresqlRepository {
    pool: PgPool,
}

impl DomainRepository for SqlxPostgresqlRepository {
    async fn new() -> Self {
        let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        debug!("[DB] Connecting to {}", url);
        let pool = PgPool::connect(&url).await.unwrap();
        debug!("[DB] Connected to {}", url);

        Self { pool }
    }
}
