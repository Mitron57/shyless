use crate::application::dto::Credentials;
use crate::domain::interfaces;
use crate::domain::interfaces::UserRepository;
use crate::domain::models::User;
use axum::async_trait;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::error::Error;
use std::io;
use std::io::ErrorKind;
use std::sync::Arc;
use tokio_postgres::types::ToSql;

pub struct JwtService {
    secret: Hmac<Sha256>,
}

impl JwtService {
    pub fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let secret = if let Ok(secret) = std::env::var("JWT_SECRET") {
            secret
        } else {
            return Err("JWT_SECRET must be set".into());
        };
        let secret = Hmac::new_from_slice(secret.as_bytes())?;
        Ok(Self { secret })
    }
}
#[async_trait]
impl interfaces::AuthService for JwtService {
    type Error = Box<dyn Error + Send + Sync>;
    type Credentials = Credentials;
    type Repository = Arc<dyn UserRepository<Self::Error, String, Box<dyn ToSql + Send + Sync>>>;

    async fn login(
        &self,
        credentials: &Self::Credentials,
        repo: Self::Repository,
    ) -> Result<String, Self::Error> {
        if let Some(user) = repo.get(credentials).await? {
            if user.password == credentials.password {
                Ok(user.sign_with_key(&self.secret)?)
            } else {
                Err("Incorrect password".into())
            }
        } else {
            Err(io::Error::from(ErrorKind::NotFound).into())
        }
    }

    async fn register(
        &self,
        credentials: &Self::Credentials,
        repo: Self::Repository,
    ) -> Result<String, Self::Error> {
        if repo.get(credentials).await?.is_some() {
            Err(io::Error::from(ErrorKind::AlreadyExists).into())
        } else {
            let mut user = User {
                id: 0,
                login: credentials.login.clone(),
                password: credentials.password.clone(),
            };
            let id = repo.add(user.clone()).await?;
            user.id = id;
            Ok(user.sign_with_key(&self.secret)?)
        }
    }

    async fn parse(&self, token: &str) -> Result<User, Self::Error> {
        Ok(token.verify_with_key(&self.secret)?)
    }

    async fn validate(&self, user: &User, repo: Self::Repository) -> Result<(), Self::Error> {
        let credentials = Credentials {
            login: user.login.clone(),
            password: user.password.clone(),
        };
        if repo.get(&credentials).await?.is_some() {
            Ok(())
        } else {
            Err(io::Error::from(ErrorKind::NotFound).into())
        }
    }
}
