# Database Manager

## Purpose

This crate makes it easy for database providers and web framework providers
to create databases for testing and tear them down at the end of the test.

This provides the facility to offer "transactional" integration tests, at least
in the frame of keeping data hygienic.

## Roadmap
### 0.1 - Postgres and integration with Rocket
- [x] support creating Postgres databases
- [x] integrate with Rocket

### 0.2 - support spinning up multiple Diesel Postgres databases with single manager
- [ ] support multiple database creations and teardowns

### 0.3 - Expanding Diesel backend support
- [ ] support creating Diesel MySQL databases
- [ ] support creating Diesel SQLite databases
  - Not sure on this one. It might be solvable by just injecting `:memory:` as
  the database type for SQLite.

### 0.4+ - expand connection support to other database libraries and async drivers
- TODO: Add a list of databases to target
