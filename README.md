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

Implementation options:
* askama-rs: thymeleaf like template framework

---
Witty phrase goes here
