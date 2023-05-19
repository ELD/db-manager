use rocket::{async_trait, figment::Figment};
use rocket_db_pools::Pool;

#[async_trait]
pub trait TestStandPool: Pool + Send + Sync + 'static {
    async fn create_database(config: &Figment) -> Result<String, <Self as Pool>::Error>;

    async fn migrate_database(database_name: &str, config: &Figment)
        -> Result<String, Self::Error>;
}

mod sqlx {
    use rocket::{async_trait, figment::providers::Serialized, info};
    use rocket_db_pools::{Config, Error, Pool};
    use sqlx::Postgres;
    use uuid::Uuid;

    #[async_trait]
    impl crate::TestStandPool for sqlx::Pool<Postgres> {
        async fn create_database(
            config: &rocket::figment::Figment,
        ) -> Result<String, <Self as rocket_db_pools::Pool>::Error> {
            let parsed_config = config.extract::<Config>()?;
            let database =
                &parsed_config.url[parsed_config.url.rfind("/").unwrap_or_default() + 1..];
            let pool = <Self as Pool>::init(config).await?;

            let temp_db = format!("{}_{}", database, Uuid::new_v4());
            info!("creating temp database: {}", temp_db);
            sqlx::query(&format!(r#"CREATE DATABASE "{}""#, temp_db,))
                .execute(&pool)
                .await
                .map_err(Error::Init)?;

            Ok(temp_db)
        }

        async fn migrate_database(
            database_name: &str,
            config: &rocket::figment::Figment,
        ) -> Result<String, Self::Error> {
            let mut url: String = config.extract_inner("url").unwrap_or_default();
            url.replace_range(url.rfind("/").unwrap_or_default() + 1.., database_name);
            let new_config = config.clone().merge(Serialized::default("url", &url));
            let pool = <Self as Pool>::init(&new_config).await?;
            sqlx::migrate!("../migrations/").run(&pool).await.unwrap();
            Ok(url)
        }
    }
}
