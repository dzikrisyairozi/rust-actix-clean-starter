use async_trait::async_trait;

#[async_trait]
pub trait UseCase<Input, Output, Error> {
    async fn execute(&self, input: Input) -> Result<Output, Error>;
}