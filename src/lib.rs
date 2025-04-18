use config::app_config::AppConfig;
use error::app_error::AppError;
use sqlx::{Executor, PgPool};
use sqlx_db_tester::TestPg;
use std::{ops::Deref, path::Path, sync::Arc};
use utils::jwt::{DecodingKey, EncodingKey};

mod config;
mod error;
mod model;
mod utils;

#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
}

pub struct AppStateInner {
    pub pool: PgPool,
    pub config: AppConfig,
    pub ek: EncodingKey,
    pub dk: DecodingKey,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, AppError> {
        let pool = PgPool::connect(&config.server.db_url).await?;

        let ek = EncodingKey::load(&config.auth.ek)?;
        let dk = DecodingKey::load(&config.auth.dk)?;
        Ok(Self {
            inner: Arc::new(AppStateInner::new(pool, config, ek, dk)),
        })
    }

    pub async fn new_for_test() -> Result<(TestPg, Self), AppError> {
        let config = AppConfig::new()?;
        let encoding_pem = include_str!("../fixtures/encoding.pem");
        let decoding_pem = include_str!("../fixtures/decoding.pem");
        let ek = EncodingKey::load(encoding_pem)?;
        let dk = DecodingKey::load(decoding_pem)?;

        let post = config
            .server
            .db_url
            .rfind('/')
            .expect("Database url should invalid");

        let database_url = &config.server.db_url[..post];
        let tdb = TestPg::new(database_url.to_string(), Path::new("./migrations"));
        let pool = tdb.get_pool().await;

        // run prepared sql to insert test dat
        let sql = include_str!("../fixtures/test.sql").split(';');
        let mut ts = pool.begin().await.expect("begin transaction failed");
        for s in sql {
            if s.trim().is_empty() {
                continue;
            }
            ts.execute(s).await.expect("execute sql failed");
        }
        ts.commit().await.expect("commit transaction failed");

        Ok((
            tdb,
            Self {
                inner: Arc::new(AppStateInner {
                    config,
                    pool,
                    ek,
                    dk,
                }),
            },
        ))
    }
}

impl AppStateInner {
    pub fn new(pool: PgPool, config: AppConfig, ek: EncodingKey, dk: DecodingKey) -> Self {
        Self {
            pool,
            config,
            ek,
            dk,
        }
    }
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
