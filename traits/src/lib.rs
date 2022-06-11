use async_trait::async_trait;
#[async_trait]
pub trait AsyncSender<T> {
    type Error;
    async fn send(&self, input: &T) -> Result<(), Self::Error>;
}
