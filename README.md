# Rocket Test Stand

A create for spinning up temporary databases for testing purpose.

## Basic Strategy

1. Call `test_stand()` function on database type
1. `test_stand()` returns a `Fairing` instance
1. `on_attach`:
   1. retrieves the DDL/test stand configuration to create a connection (macro attribute!)
   1. creates the DDL connection
   1. creates a database with a UUID appended
   1. Run migrations (macro attribute!)
   1. Merge the database name with the main connection info
   1. Change the pool connection for the primary database connection (macro attribute!)
   1. Continue
