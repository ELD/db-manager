use diesel::connection::SimpleConnection;
use diesel::{Connection, PgConnection};
use uuid::Uuid;

table! {
    pg_database (datname) {
        datname -> Text,
        datistemplate -> Bool,
    }
}

pub struct DatabaseManager {
    connection: diesel::PgConnection,
    database_name: Option<String>,
}

impl DatabaseManager {
    pub fn new<S: Into<String>>(database_url: S) -> Self {
        let connection = PgConnection::establish(&database_url.into()).expect("connection");
        Self {
            connection,
            database_name: None,
        }
    }

    // TODO: Only work if a database hasn't been created yet
    pub fn create_database(&mut self) -> Option<String> {
        let database_name = self.random_database_name();
        match self
            .connection
            .batch_execute(&format!("CREATE DATABASE {}", database_name))
        {
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

        pg_database
            .select(datname)
            .filter(datname.eq(database_name.into()))
            .filter(datistemplate.eq(false))
            .get_result::<String>(&self.connection)
            .optional()
            .map(|res| res.is_some())
            .unwrap()
    }

    pub fn get_connection_url() -> String {
        todo!("this should allow us to export the connection info to another entity, i.e. Rocket config")
    }

    fn random_database_name(&self) -> String {
        format!("test_db_{}", Uuid::new_v4().to_simple().to_string())
    }
}

impl Drop for DatabaseManager {
    fn drop(&mut self) {
        if let Some(database_name) = self.database_name.clone() {
            self.connection
                .batch_execute(&format!("DROP DATABASE IF EXISTS {}", database_name))
                .expect("unable to drop database");
        }
    }
}
