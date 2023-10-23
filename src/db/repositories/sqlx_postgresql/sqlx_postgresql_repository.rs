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
        let names_non_mut = names.clone();

        let known_domains = sqlx::query!(r#"
                SELECT domain FROM private_keys
                INNER JOIN payment_inscription_contents ON payment_inscription_contents.id = private_keys.payment_inscription_content_id 
                INNER JOIN payments ON payments.id = payment_inscription_contents.payment_id 
                WHERE payments.initiated = True 
                    AND private_keys.domain = ANY($1);
            "#,
            &names_non_mut
            )
            .fetch_all(&self.pool)
            .await
            .unwrap();

        let known_domains: Vec<String> = known_domains.into_iter().map(|row| row.domain).collect();
        names.retain(|name| !known_domains.contains(name));

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

    async fn get_domains_of_addresses(&self, addresses: &Vec<String>) -> Vec<(String, String)> {
        debug!("[DB] Getting domains of addresses {:?}", addresses);

        let res = sqlx::query!(
            r#"
                SELECT name, inscription
                FROM domain
                WHERE address = ANY($1)
            "#,
            addresses
        )
        .fetch_all(&self.pool)
        .await;

        match res {
            Ok(res) => res
                .into_iter()
                .map(|row| (row.name, row.inscription))
                .collect(),
            Err(err) => {
                debug!("[DB] Error getting domains of addresses: {:?}", err);
                vec![]
            }
        }
    }
}
