# dashboard
Very simple dashboard implemented with Rust.

## Applications
This project workspace consists of 4 projects:
* [common](/common): _Shared resources for the dashboard workspace_
* [dashboard-data](/dashboard-data): _Simple terminal program to generate/update the database_
* [dashboard-rest](/dashboard-rest): _REST api for access to database_
* [dashboard-ui](/dashboard-ui): _Display a simple web ui containing status of all applications_

## Exposed services
* rest: `./run-rest`
  * http://127.0.0.1:8080/maintainer/apps
  * http://127.0.0.1:8080/version
  * http://127.0.0.1:8080/ping
* ui: `./run-ui.sh`
  * http://127.0.0.1:8081/dashboard
  * http://127.0.0.1:8081/version
  * http://127.0.0.1:8081/ping

---
_Witty phrase goes here._
