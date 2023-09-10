pub trait DomainRepository
where
    Self: Clone,
{
    async fn new() -> Self;
}
