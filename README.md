# Database Manager

## Purpose

This crate makes it easy for database providers and web framework providers
to create databases for testing and tear them down at the end of the test.

This provides the facility to offer "transactional" integration tests, at least
in the frame of keeping data hygienic.

## Roadmap
### MVP - singleton database manager
- [ ] support creating Postgres databases
- [ ] support creating MySQL databases
- [ ] support creating SQLite databases

### Phase one - integration with web frameworks
- [ ] integrate with Rocket

### Phase two - support spinning up multiple databases with single manager
- [ ] support multiple database creations and teardowns

### Phase three - expand connection support to other database libraries
- TODO: Add a list of databases to target
