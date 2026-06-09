# dashboard
Building a very simple dashboard in Rust.

Implemented as a rust workspace with 3 binaries and one shared library.

## Applications
### Common
Provides shared object model.

### Database
Simple application that runs once to initialize or migrate/upgrade database.

Uses:
* [rusqlite](https://docs.rs/rusqlite/latest/rusqlite/): connect to sqlite database
* [rusqlite_migration](https://docs.rs/rusqlite_migration/latest/rusqlite_migration/): liquibase-like migration and versioned ddl
* [include_dir](https://docs.rs/include_dir/latest/include_dir/): supports rusqlite migration by importing an entire directory containing ddl migrations

### Rest API
REST api for access to database.

Uses:
* [rusqlite](https://docs.rs/rusqlite/latest/rusqlite/): connect to sqlite database
* [rocket](https://rocket.rs/): web service support
* [serde_json](https://docs.rs/serde_json/latest/serde_json/): return data as json

### User Interface
Display a simple web ui containing status of all applications.

Uses:
* [rocket](https://rocket.rs/): web service support
* [askama](https://askama.rs/en/stable/introduction.html): web templates
* [reqwest](https://docs.rs/reqwest/latest/reqwest/): http client

## Exposed services
* rest: `./run-rest` →  http://127.0.0.1:8080/maintainer/apps
* ui: `./run-ui.sh`  →  http://127.0.0.1:8081/dashboard

---
Witty phrase goes here
