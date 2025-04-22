use crate::{AppState, error::app_error::AppError, utils::password::hash_password};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

impl AppState {
    pub async fn create_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<User, AppError> {
        if self.get_user_by_email(email).await?.is_some() {
            return Err(AppError::UserAlreadyExists(email.to_string()));
        }

        let password_hash = hash_password(password)?;

        let user = sqlx::query_as(
            "
            insert into users (username, email, password_hash) values ($1, $2, $3) returning *
            ",
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as(
            "
            select * from users where email = $1
            ",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn delete_user_by_email(&self, email: &str) -> Result<(), AppError> {
        sqlx::query(
            "
            delete from users where email = $1
            ",
        )
        .bind(email)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn change_user_password(
        &self,
        password: &str,
        email: &str,
    ) -> Result<User, AppError> {
        let password_hash = hash_password(password)?;

        let user = sqlx::query_as(
            "
            update users set password_hash = $1 where email = $2 returning *
            ",
        )
        .bind(password_hash)
        .bind(email)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
}

impl User {
    pub fn new(username: &str, email: &str, password_hash: &str) -> Self {
        Self {
            id: 0,
            username: username.to_string(),
            email: email.to_string(),
            password_hash: password_hash.to_string(),
            created_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::password::verify_password;

    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn create_user_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let username = "TeamMeng";
        let email = "Meng@123.com";
        let password = "123456";
        let user = state.create_user(username, email, password).await?;

        assert_eq!(user.username, username);
        assert_eq!(user.email, email);
        Ok(())
    }

    #[tokio::test]
    async fn get_user_by_email_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let username = "Test Meng";
        let email = "Test@123.com";

        let user = state
            .get_user_by_email(email)
            .await?
            .expect("user should exists");

        assert_eq!(user.username, username);
        assert_eq!(user.email, email);

        Ok(())
    }

    #[tokio::test]
    async fn delete_user_by_email_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let email = "Test@123.com";

        let ret = state.delete_user_by_email(email).await;

        assert!(ret.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn change_user_password_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let password = "hunter42";
        let email = "Test@123.com";

        let user = state.change_user_password(password, email).await?;

        assert!(verify_password(password, &user.password_hash)?);

        Ok(())
    }
}
