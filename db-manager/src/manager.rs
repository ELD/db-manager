use diesel::connection::SimpleConnection;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use url::Url;
use uuid::Uuid;

table! {
    pg_database (datname) {
        datname -> Text,
        datistemplate -> Bool,
    }
}

pub struct DatabaseManager {
    database_url: Url,
    connection_pool: Pool<ConnectionManager<PgConnection>>,
    database_name: Option<String>,
}

impl DatabaseManager {
    pub fn new<S: Into<String>>(database_url: S) -> Self {
        let database_url = Url::parse(&database_url.into())
            .unwrap()
            .join("postgres")
            .unwrap();
        let manager = ConnectionManager::new(database_url.to_string());
        let connection = Pool::builder().max_size(1).build(manager).unwrap();
        Self {
            database_url,
            connection_pool: connection,
            database_name: None,
        }
    }

    // TODO: Only work if a database hasn't been created yet
    pub fn create_database(&mut self) -> Option<String> {
        let connection = self.connection_pool.get().unwrap();
        let database_name = self.random_database_name();
        match (*connection).batch_execute(&format!("CREATE DATABASE {}", database_name)) {
            Ok(_) => {
                self.database_name = Some(database_name);
                self.database_name.clone()
            }
            Err(e) => {
                eprintln!("{:?}", e);
                None
            }
        }
    }

    pub fn database_exists<S: Into<String>>(&self, database_name: S) -> bool {
        use crate::manager::pg_database::dsl::*;
        use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};

        let connection = self.connection_pool.get().unwrap();

        pg_database
            .select(datname)
            .filter(datname.eq(database_name.into()))
            .filter(datistemplate.eq(false))
            .get_result::<String>(&*connection)
            .optional()
            .map(|res| res.is_some())
            .unwrap()
    }

    pub fn get_connection_url(&self) -> String {
        self.database_url
            .join(self.database_name.clone().unwrap().as_ref())
            .unwrap()
            .to_string()
    }

    fn random_database_name(&self) -> String {
        format!("test_db_{}", Uuid::new_v4().to_simple().to_string())
    }
}

impl Drop for DatabaseManager {
    fn drop(&mut self) {
        if let Some(database_name) = self.database_name.clone() {
            let connection = self.connection_pool.get().unwrap();

            // Force disconnection to all connected clients
            (*connection)
                .batch_execute(&format!("SELECT pg_terminate_backend(pg_stat_activity.pid) FROM pg_stat_activity WHERE pg_stat_activity.datname = '{}' AND pid <> pg_backend_pid()", database_name))
                .expect("unable to disconnect clients");

            // Drop the database
            (*connection)
                .batch_execute(&format!("DROP DATABASE IF EXISTS {}", database_name))
                .expect("unable to drop database");
        }
    }
}
