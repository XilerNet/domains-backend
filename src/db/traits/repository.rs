pub trait DomainRepository
where
    Self: Clone,
{
    async fn new() -> Self;
    async fn retain_available_domain_names(&self, names: &mut Vec<String>);
}
