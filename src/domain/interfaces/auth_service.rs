use crate::domain::models::User;
use axum::async_trait;

#[async_trait]
pub trait AuthService {
    type Error;
    type Credentials;

    type Repository: Send;

    async fn login(
        &self,
        credentials: &Self::Credentials,
        repo: Self::Repository,
    ) -> Result<String, Self::Error>;
    async fn register(
        &self,
        credentials: &Self::Credentials,
        repo: Self::Repository,
    ) -> Result<String, Self::Error>;

    async fn parse(&self, token: &str) -> Result<User, Self::Error>;

    async fn validate(&self, user: &User, repo: Self::Repository) -> Result<(), Self::Error>;

    async fn parse_and_validate(
        &self,
        token: &str,
        repo: Self::Repository,
    ) -> Result<User, Self::Error> {
        let user = self.parse(token).await?;
        self.validate(&user, repo).await?;
        Ok(user)
    }
}
