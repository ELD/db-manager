use crate::manager::DatabaseManager;
use rocket::config::Value;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::Rocket;

pub struct RocketDatabaseManager {
    config_name: String,
}

impl RocketDatabaseManager {
    pub fn new(config_name: String) -> Self {
        Self { config_name }
    }
}

impl Fairing for RocketDatabaseManager {
    fn info(&self) -> Info {
        Info {
            name: "Database Manager Fairing",
            kind: Kind::Attach,
        }
    }

    fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        let mut databases = rocket.config().get_table("databases").unwrap().clone();
        let mut database_config = databases
            .get(&self.config_name)
            .unwrap()
            .as_table()
            .unwrap()
            .clone();

        let base_url = database_config.get("url").unwrap().as_str().unwrap();
        let mut manager = DatabaseManager::new(base_url);
        manager.create_database().unwrap();
        let database_url = manager.get_connection_url();

        database_config.insert("url".to_string(), Value::String(database_url));
        databases.insert(
            self.config_name.clone(),
            Value::Table(database_config.clone()),
        );

        let extras = rocket
            .config()
            .extras()
            .map(|(key, value)| {
                let value = if key == "databases" {
                    Value::Table(databases.clone())
                } else {
                    value.clone()
                };

                (key.to_string(), value)
            })
            .collect();

        let mut config = rocket.config().clone();
        config.set_extras(extras);

        // FIXME: The use of `rocket::custom` overrides any other settings set up to this point. This could squash a user's other fairings or routes. It must be the first method on Rocket they call or they could lose everything.
        Ok(rocket::custom(config).manage(manager))
    }
}
