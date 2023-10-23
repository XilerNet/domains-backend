pub trait DomainRepository
where
    Self: Clone,
{
    async fn new() -> Self;
    async fn retain_available_domain_names(&self, names: &mut Vec<String>);
    async fn get_domains_of_addresses(&self, addresses: &Vec<String>) -> Vec<(String, String)>;
    async fn get_domain_by_inscription(
        &self,
        inscription: &str,
    ) -> Result<Option<String>, sqlx::Error>;
}
