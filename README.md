# dashboard
Building a very simple dashboard in Rust.

Implemented as a rust workspace with 3 binaries and one shared library.

## Applications
### Common
Provides shared object model.

### Database
Simple application that runs once to initialize or migrate/upgrade database.

Uses:
* rusqlite: connect to sqlite database
* rusqlite_migration: liquibase-like migration and versioned ddl
* include_dir: supports rusqlite migration by importing an entire directory containing ddl migrations

### Rest API
REST api for access to database.

Uses:
* rusqlite: connec to sqlite database
* rocket: web service support
* serde_json: return data as json

### User Interface
Display a simple web ui containing status of all applications.

Uses:
* rocket: web service support
* [askama](https://askama.rs/en/stable/introduction.html): web templates

Exposed services:
* rest: `cargo run --bin dashboard-rest` → http://127.0.0.1:8000/maintainer/apps?owner=fronk
* ui: `./run.sh`  →  http://127.0.0.1:8000/dashboard

---
Witty phrase goes here
