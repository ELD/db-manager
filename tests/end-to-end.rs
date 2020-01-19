use dbmanager::manager::DatabaseManager;

#[test]
fn it_creates_a_postgres_database_with_random_name() {
    let mut manager = DatabaseManager::new("postgres://edattore@localhost/postgres");
    let db_name = manager.create_database().unwrap();

    assert!(manager.database_exists(db_name));
}

#[test]
fn it_drops_the_postgres_databases_it_creates() {
    let mut manager = DatabaseManager::new("postgres://edattore@localhost/postgres");
    let db_name = manager.create_database().unwrap();

    drop(manager);

    manager = DatabaseManager::new("postgres://edattore@localhost/postgres");
    assert!(!manager.database_exists(db_name));
}
