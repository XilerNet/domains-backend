use sqlx::PgPool;
use tracing::debug;

use crate::db::DomainRepository;

const DOMAIN_LIFETIME: i64 = 31536000;

// TODO: Handle multiple databases
#[derive(Clone)]
pub struct SqlxPostgresqlRepository {
    pool: PgPool,
}

impl DomainRepository for SqlxPostgresqlRepository {
    async fn new() -> Self {
        let url =
            std::env::var("DOMAIN_BTC_DATABASE_URL").expect("DOMAIN_BTC_DATABASE_URL must be set");

        debug!("[DB] Connecting to {}", url);
        let pool = PgPool::connect(&url).await.unwrap();
        debug!("[DB] Connected to {}", url);

        Self { pool }
    }

    async fn retain_available_domain_names(&self, names: &mut Vec<String>) {
        let mut index = 0;
        let mut max_index = names.len() - 1;

        while index <= max_index {
            if names.len() == 0 {
                break;
            }

            let domain = sqlx::query!(
                r#"
                    SELECT name, inscription, valid_from
                    FROM domain
                    WHERE name = $1
                    ORDER BY valid_from DESC
                    LIMIT 1
                "#,
                names[index]
            )
            .fetch_optional(&self.pool)
            .await
            .unwrap();

            if let Some(domain) = domain {
                let valid_from = domain.valid_from;
                let now = chrono::Utc::now().timestamp();
                let valid_from: i64 = valid_from.parse().unwrap();

                if valid_from + DOMAIN_LIFETIME > now {
                    names.remove(index);

                    if max_index > 0 {
                        max_index -= 1;
                    }
                    continue;
                }
            }

            index += 1;
        }
    }
}
