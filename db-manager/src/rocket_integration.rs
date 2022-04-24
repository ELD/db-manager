use crate::manager::DatabaseManager;
use rocket::fairing::{Fairing, Info, Kind, Result};
use rocket::figment::providers::Serialized;
use rocket::{Build, Rocket};
use rocket_sync_db_pools::Config;

pub struct RocketDatabaseManager {
    config_name: String,
}

impl RocketDatabaseManager {
    pub fn new(config_name: String) -> Self {
        Self { config_name }
    }
}

#[async_trait::async_trait]
impl Fairing for RocketDatabaseManager {
    fn info(&self) -> Info {
        Info {
            name: "Database Manager Fairing",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result {
        let current_config = Config::from(&self.config_name, &rocket).unwrap();
        let mut manager = DatabaseManager::new(current_config.url).unwrap();

        let handle = rocket::tokio::task::spawn_blocking(|| {
            manager.create_database().unwrap();
            manager
        });

        let manager = handle.await.unwrap();

        let updated_config = rocket_sync_db_pools::Config {
            url: manager.get_connection_url(),
            pool_size: 10,
            timeout: 5,
        };

        let figment = rocket.figment().clone().merge(Serialized::global(
            &format!("databases.{}", &self.config_name),
            updated_config,
        ));

        // // FIXME: The use of `rocket::custom` overrides any other settings set up to this point. This could squash a user's other fairings or routes. It must be the first method on Rocket they call or they could lose everything.
        // Ok(rocket::custom(config).manage(manager))
        Ok(rocket.configure(figment).manage(manager))
    }
}
