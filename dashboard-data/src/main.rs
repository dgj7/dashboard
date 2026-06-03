use include_dir::{include_dir, Dir};
use rusqlite::Connection;
use rusqlite_migration::Migrations;

static DDL: Dir = include_dir!("$CARGO_MANIFEST_DIR/ddl");

fn main() {
    let mut connection = Connection::open("apps.db").unwrap();
    let migrations = Migrations::from_directory(&DDL).unwrap();
    migrations.to_latest(&mut connection).unwrap();
}
