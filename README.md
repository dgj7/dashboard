# dashboard
Building a very simple dashboard in Rust.

Implemented as a rust workspace with 3 binaries and one shared library.

## Applications
### Database
The database application is needed for storing analytics.

Implementation options:
* diesel
* sea orm
* surreal-db
* rusqlite: option for storing database data in a local data file

### Rest API
Provides rest api that the ui can call for data.

Probably will use Rocket.

### User Interface
Display a simple web ui containing status of all applications.

Frameworks used:
* rocket: web service support
* askama: web templates

Exposed services:
* `cargo run --bin dashboard-ui`  →  http://127.0.0.1:8000/shrek

---
Witty phrase goes here
