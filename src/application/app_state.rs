use crate::application::dto::Credentials;
use crate::domain::interfaces::{self, Repository, ToKey, UserRepository};
use crate::domain::models::Post;
use std::error;
use std::sync::Arc;
use tokio_postgres::types;

type ToSql = dyn types::ToSql + Sync + Send;
type Error = Box<dyn error::Error + Send + Sync>;
type UserRepo = dyn UserRepository<Error, String, Box<ToSql>>;
type PostRepo = dyn Repository<Post, Key = dyn ToKey<Key = ToSql>, Error = Error>;

type AuthService = dyn interfaces::AuthService<Credentials = Credentials, Error = Error, Repository = Arc<UserRepo>>
    + Send
    + Sync;

type PostService =
    dyn interfaces::PostService<Repository = Arc<PostRepo>, Error = Error> + Send + Sync;

pub struct AppState {
    pub post_service: Arc<PostService>,
    pub user_repository: Arc<UserRepo>,
    pub post_repository: Arc<PostRepo>,
    pub auth_service: Arc<AuthService>,
}
